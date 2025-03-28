use serde::{Deserialize, Serialize};

mod clock;

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

/// 3D model struct
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Model {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gltf: Option<String>,
}

/// Orientation
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Orientation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_quaternion: Option<[f64; 4]>,
}
