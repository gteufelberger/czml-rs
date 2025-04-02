/// Primitive datatypes used by different objects
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum Origin {
    #[serde(rename = "CENTER")]
    Center,
    #[serde(rename = "LEFT")]
    Left,
}
