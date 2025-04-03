/// Primitive datatypes used by different objects
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ArcType {
    None,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Cartesian {
    Timestamp(String),
    Coordinate(f64),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InterpolationAlgorithms {
    Lagrange,
}

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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReferenceFrames {
    Inertial,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ShowInterval {
    pub interval: String,
    pub boolean: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum StringOrVec {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum BooleanOrShowObjectVec {
    Single(bool),
    Multiple(Vec<ShowInterval>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Color {
    pub rgba: [u8; 4],
}
