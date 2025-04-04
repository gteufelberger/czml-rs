use serde::{Deserialize, Serialize};

pub mod packet;

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

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "agi_rectangularSensor")]
    pub agi_rectangular_sensor: Option<packet::AgiRectangularSensor>,

    /// Time interval of validity (ISO 8601 string or array of intervals)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<packet::type_enums::StringOrVec>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billboard: Option<packet::Billboard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock: Option<packet::Clock>,

    /// A plain text or HTML description of the packet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<packet::Label>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<packet::Model>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<packet::Orientation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<packet::Path>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<packet::Point>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon: Option<packet::Polygon>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub polyline: Option<packet::Polyline>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<packet::Position>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_from: Option<packet::ViewFrom>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_most_basic_packet() {
        let _ = Packet {
            id: "document".to_string(),
            name: None,
            version: Some("1.0".to_string()),
            agi_rectangular_sensor: None,
            availability: None,
            billboard: None,
            clock: None,
            description: None,
            label: None,
            model: None,
            orientation: None,
            parent: None,
            path: None,
            point: None,
            polygon: None,
            polyline: None,
            position: None,
            view_from: None,
            properties: None,
        };
    }
}
