use bencher_json::{project::threshold::JsonStatistic, ResourceId};
use diesel::{BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use dropshot::{endpoint, HttpError, Path, RequestContext};
use schemars::JsonSchema;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    context::ApiContext,
    endpoints::{
        endpoint::{pub_response_ok, response_ok, ResponseOk},
        Endpoint, Method,
    },
    error::api_error,
    model::project::{threshold::statistic::QueryStatistic, QueryProject},
    model::user::auth::AuthUser,
    schema,
    util::cors::{get_cors, CorsResponse},
    ApiError,
};

use super::Resource;

const STATISTIC_RESOURCE: Resource = Resource::Statistic;

#[derive(Deserialize, JsonSchema)]
pub struct OnePath {
    pub project: ResourceId,
    pub statistic: Uuid,
}

#[allow(clippy::unused_async)]
#[endpoint {
    method = OPTIONS,
    path =  "/v0/projects/{project}/statistics/{statistic}",
    tags = ["projects", "statistics"]
}]
pub async fn one_options(
    _rqctx: RequestContext<ApiContext>,
    _path_params: Path<OnePath>,
) -> Result<CorsResponse, HttpError> {
    Ok(get_cors::<ApiContext>())
}

#[endpoint {
    method = GET,
    path =  "/v0/projects/{project}/statistics/{statistic}",
    tags = ["projects", "statistics"]
}]
pub async fn get_one(
    rqctx: RequestContext<ApiContext>,
    path_params: Path<OnePath>,
) -> Result<ResponseOk<JsonStatistic>, HttpError> {
    let auth_user = AuthUser::new(&rqctx).await.ok();
    let endpoint = Endpoint::new(STATISTIC_RESOURCE, Method::GetOne);

    let json = get_one_inner(
        rqctx.context(),
        path_params.into_inner(),
        auth_user.as_ref(),
    )
    .await
    .map_err(|e| endpoint.err(e))?;

    if auth_user.is_some() {
        response_ok!(endpoint, json)
    } else {
        pub_response_ok!(endpoint, json)
    }
}

async fn get_one_inner(
    context: &ApiContext,
    path_params: OnePath,
    auth_user: Option<&AuthUser>,
) -> Result<JsonStatistic, ApiError> {
    let conn = &mut *context.conn().await;

    let query_project =
        QueryProject::is_allowed_public(conn, &context.rbac, &path_params.project, auth_user)?;

    schema::statistic::table
        .left_join(
            schema::threshold::table.on(schema::statistic::id.eq(schema::threshold::statistic_id)),
        )
        .left_join(schema::testbed::table.on(schema::threshold::testbed_id.eq(schema::testbed::id)))
        .filter(
            schema::testbed::project_id
                .eq(query_project.id)
                .and(schema::statistic::uuid.eq(path_params.statistic.to_string())),
        )
        .select((
            schema::statistic::id,
            schema::statistic::uuid,
            schema::statistic::test,
            schema::statistic::min_sample_size,
            schema::statistic::max_sample_size,
            schema::statistic::window,
            schema::statistic::left_side,
            schema::statistic::right_side,
        ))
        .first::<QueryStatistic>(conn)
        .map_err(api_error!())?
        .into_json()
}
