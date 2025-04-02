use serde::{Deserialize, Serialize};

mod billboard;
mod clock;
mod label;
mod primitives;

/// A billboard is a viewport-aligned image positioned in the 3D scene
/// https://cesium.com/learn/cesiumjs/ref-doc/Billboard.html
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Billboard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eye_offset: Option<billboard::EyeOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixel_offset: Option<billboard::PixelOffset>,
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
    pub show: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixel_offset: Option<billboard::PixelOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_color: Option<primitives::Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_origin: Option<primitives::Origin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_origin: Option<primitives::Origin>,
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
    pub minimum_pixel_size: Option<u32>,
}

/// Orientation
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Orientation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_quaternion: Option<[f64; 4]>,
}
