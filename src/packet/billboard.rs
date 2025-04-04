use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EyeOffset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartesian: Option<Vec<f64>>,
}
