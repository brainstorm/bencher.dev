use derive_more::Display;
use git_validate::reference::name_partial;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use std::{fmt, str::FromStr};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::ValidError;

#[derive(Debug, Display, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct BranchName(String);

impl FromStr for BranchName {
    type Err = ValidError;

    fn from_str(branch_name: &str) -> Result<Self, Self::Err> {
        if is_valid_branch_name(branch_name) {
            Ok(Self(branch_name.into()))
        } else {
            Err(ValidError::BranchName(branch_name.into()))
        }
    }
}

impl AsRef<str> for BranchName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<BranchName> for String {
    fn from(branch_name: BranchName) -> Self {
        branch_name.0
    }
}

impl<'de> Deserialize<'de> for BranchName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(BranchNameVisitor)
    }
}

struct BranchNameVisitor;

impl<'de> Visitor<'de> for BranchNameVisitor {
    type Value = BranchName;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid branch name")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.parse().map_err(E::custom)
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn is_valid_branch_name(branch_name: &str) -> bool {
    name_partial(branch_name.into()).is_ok()
}

#[cfg(test)]
mod test {
    use super::is_valid_branch_name;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_email() {
        assert_eq!(true, is_valid_branch_name("refs/heads/main"));
        assert_eq!(true, is_valid_branch_name("main"));
        assert_eq!(true, is_valid_branch_name("MAIN"));
        assert_eq!(true, is_valid_branch_name("bencher/main"));

        assert_eq!(false, is_valid_branch_name(""));
        assert_eq!(false, is_valid_branch_name(" main"));
        assert_eq!(false, is_valid_branch_name("ma in"));
        assert_eq!(false, is_valid_branch_name("main "));
        assert_eq!(false, is_valid_branch_name(".main"));
    }
}
