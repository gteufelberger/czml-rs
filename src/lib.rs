use serde::{Deserialize, Serialize};

mod packet;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Packet {
    pub id: String,

    /// Optional human-readable name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Version field (only on the document packet)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Time interval of validity (ISO 8601 string or array of intervals)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billboard: Option<Billboard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock: Option<packet::Clock>,

    /// A plain text or HTML description of the packet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<packet::Model>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<packet::Orientation>,
}

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
    pub eye_offset: Option<EyeOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixel_offset: Option<PixelOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_origin: Option<HorizontalOrigin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_origin: Option<VerticalOrigin>,
}

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
pub enum HorizontalOrigin {
    #[serde(rename = "CENTER")]
    Center,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum VerticalOrigin {
    #[serde(rename = "CENTER")]
    Center,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
