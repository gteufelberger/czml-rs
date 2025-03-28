use serde::{Deserialize, Serialize};

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
