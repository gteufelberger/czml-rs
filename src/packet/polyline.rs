use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SolidColor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<super::primitives::Color>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PolylineOutline {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<super::primitives::Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_width: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Position {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartesian: Option<Vec<super::primitives::Cartesian>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartographic_radians: Option<Vec<f64>>,
}
