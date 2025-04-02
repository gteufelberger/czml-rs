/// Primitive datatypes used by different objects
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum Origin {
    Center,
    Left,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PixelOffset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartesian2: Option<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Color {
    pub rgba: [u8; 4],
}
