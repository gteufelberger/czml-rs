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
#[serde(untagged)]
pub enum UnitQuaternion {
    Timestamp(String),
    Angle(f64),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dimensions {
    pub cartesian: [f64; 3],
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ForwardExtrapolationType {
    Hold,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InterpolationAlgorithms {
    Lagrange,
    Linear,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Material {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solid_color: Option<super::polyline::SolidColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polyline_outline: Option<super::polyline::PolylineOutline>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SurfaceMaterial {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solid_color: Option<super::polyline::SolidColor>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum Origin {
    Center,
    Left,
    Right,
    Bottom,
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
pub enum PortionToDisplay {
    Complete,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReferenceFrames {
    Inertial,
    Fixed,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ShowInterval {
    pub interval: String,
    pub boolean: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub rgba: [f64; 4],
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ColorInterval {
    pub interval: String,
    pub rgba: [u8; 4],
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct NumberInterval {
    pub interval: String,
    pub number: f64,
}
