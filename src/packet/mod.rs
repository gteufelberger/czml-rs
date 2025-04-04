use serde::{Deserialize, Serialize};

pub mod billboard;
pub mod clock;
pub mod label;
pub mod path;
pub mod polyline;
pub mod primitives;
pub mod type_enums;

/// rectangular sensor
/// https://cesium.com/learn/ion-sdk/ref-doc/RectangularSensor.html
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AgiRectangularSensor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_half_angle: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_half_angle: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<type_enums::BooleanOrShowObjectVec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portion_to_display: Option<primitives::PortionToDisplay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_intersection: Option<type_enums::BooleanOrShowObjectVec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intersection_color: Option<primitives::Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intersection_width: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipsoid_surface_material: Option<primitives::SurfaceMaterial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipsoid_horizon_surface_material: Option<primitives::SurfaceMaterial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lateral_surface_material: Option<primitives::SurfaceMaterial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dome_surface_material: Option<primitives::SurfaceMaterial>,
}

/// A billboard is a viewport-aligned image positioned in the 3D scene
/// https://cesium.com/learn/cesiumjs/ref-doc/Billboard.html
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Billboard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<type_enums::BooleanOrShowObjectVec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eye_offset: Option<billboard::EyeOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixel_offset: Option<primitives::PixelOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_origin: Option<primitives::Origin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_origin: Option<primitives::Origin>,
}

/// A clock used to drive the time-dynamic aspects of a document.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Clock {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<clock::Range>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<clock::Step>,
}

/// A text label
/// https://cesium.com/learn/cesiumjs/ref-doc/Label.html
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Label {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<type_enums::BooleanOrShowObjectVec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixel_offset: Option<primitives::PixelOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_color: Option<type_enums::ColorOrColorInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_origin: Option<primitives::Origin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_origin: Option<primitives::Origin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_color: Option<type_enums::ColorOrColorInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<label::Style>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// 3D model struct
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Model {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gltf: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_scale: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_pixel_size: Option<u32>,
}

/// Orientation
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Orientation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolation_algorithm: Option<primitives::InterpolationAlgorithms>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolation_degree: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_quaternion: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity_reference: Option<String>,
}

/// Path
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Path {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<type_enums::BooleanOrShowObjectVec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<primitives::Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<primitives::Material>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_color: Option<primitives::Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead_time: Option<type_enums::NumberOrPathLeadTrailInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_time: Option<type_enums::NumberOrPathLeadTrailInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<type_enums::FloatOrNumberObjectVec>,
}

/// Point
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Point {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<type_enums::ColorOrColorInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixel_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<type_enums::BooleanOrShowObjectVec>,
}

/// Polygon
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Polygon {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<type_enums::ColorOrColorInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<primitives::Material>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_color: Option<type_enums::ColorOrColorInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<polyline::Position>,
}

/// Polyline
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Polyline {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<type_enums::BooleanOrShowObjectVec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_type: Option<primitives::ArcType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clamp_to_ground: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<primitives::Material>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<polyline::Position>,
}

/// Position
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Position {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartesian: Option<Vec<primitives::Cartesian>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_extrapolation_type: Option<primitives::ForwardExtrapolationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolation_algorithm: Option<primitives::InterpolationAlgorithms>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolation_degree: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_frame: Option<primitives::ReferenceFrames>,
}

/// viewFrom
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ViewFrom {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartesian: Option<Vec<primitives::Cartesian>>,
}
