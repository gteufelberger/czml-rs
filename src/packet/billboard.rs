use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EyeOffset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartesian: Option<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PixelOffset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartesian2: Option<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum Origin {
    #[serde(rename = "CENTER")]
    Center,
    #[serde(rename = "LEFT")]
    Left,
}
