use bencher_rbac::{
    user::{OrganizationRoles, ProjectRoles},
    Organization, User as RbacUser,
};

use bencher_json::jwt::JsonWebToken;
use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};
use dropshot::RequestContext;
use oso::{PolarValue, ToPolar};

use crate::{
    diesel::ExpressionMethods,
    schema,
    util::{context::Rbac, Context},
    ApiError,
};

use super::macros::{auth_error, map_auth_error, org_roles_map, proj_roles_map};

const INVALID_JWT: &str = "Invalid JWT (JSON Web Token)";

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: i32,
    pub organizations: Vec<i32>,
    pub projects: Vec<i32>,
    pub rbac: RbacUser,
}

impl AuthUser {
    pub async fn new(rqctx: &RequestContext<Context>) -> Result<Self, ApiError> {
        let request = rqctx.request.lock().await;

        let headers = request
            .headers()
            .get("Authorization")
            .ok_or_else(auth_error!("Missing \"Authorization\" header"))?
            .to_str()
            .map_err(map_auth_error!("Invalid \"Authorization\" header"))?;
        let (_, token) = headers
            .split_once("Bearer ")
            .ok_or_else(auth_error!("Missing \"Authorization\" Bearer"))?;
        let jwt: JsonWebToken = token.trim().to_string().into();

        let context = &mut *rqctx.context().lock().await;
        let token_data = jwt
            .validate_user(&context.secret_key)
            .map_err(map_auth_error!(INVALID_JWT))?;

        let conn = &mut context.db_conn;
        let (user_id, admin, locked) = schema::user::table
            .filter(schema::user::email.eq(token_data.claims.email()))
            .select((schema::user::id, schema::user::admin, schema::user::locked))
            .first::<(i32, bool, bool)>(conn)
            .map_err(map_auth_error!(INVALID_JWT))?;

        let (org_ids, org_roles) = Self::organization_roles(conn, user_id)?;
        let (proj_ids, proj_roles) = Self::project_roles(conn, user_id)?;
        let rbac = RbacUser {
            admin,
            locked,
            organizations: org_roles,
            projects: proj_roles,
        };

        Ok(Self {
            id: user_id,
            organizations: org_ids,
            projects: proj_ids,
            rbac,
        })
    }

    fn organization_roles(
        conn: &mut SqliteConnection,
        user_id: i32,
    ) -> Result<(Vec<i32>, OrganizationRoles), ApiError> {
        org_roles_map!(conn, user_id)
    }

    fn project_roles(
        conn: &mut SqliteConnection,
        user_id: i32,
    ) -> Result<(Vec<i32>, ProjectRoles), ApiError> {
        proj_roles_map!(conn, user_id)
    }

    pub fn organizations(
        &self,
        rbac: &Rbac,
        action: bencher_rbac::organization::Permission,
    ) -> Vec<i32> {
        self.organizations
            .iter()
            .filter_map(|id| {
                if rbac.unwrap_is_allowed(
                    self,
                    action,
                    Organization {
                        uuid: id.to_string(),
                    },
                ) {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn projects(&self, rbac: &Rbac, action: bencher_rbac::project::Permission) -> Vec<i32> {
        // self.projects
        //     .iter()
        //     .filter_map(|id| {
        //         if rbac.unwrap_is_allowed(
        //             self,
        //             action,
        //             Project {
        //                 uuid: id.to_string(),
        //             },
        //         ) {
        //             Some(*id)
        //         } else {
        //             None
        //         }
        //     })
        //     .collect()
        Vec::new()
    }
}

impl ToPolar for &AuthUser {
    fn to_polar(self) -> PolarValue {
        self.rbac.clone().to_polar()
    }
}
