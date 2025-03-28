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
    pub billboard: Option<packet::Billboard>,

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

    #[test]
    fn create_most_basic_packet() {
        let _ = Packet {
            id: "document".to_string(),
            name: None,
            version: Some("1.0".to_string()),
            availability: None,
            billboard: None,
            clock: None,
            description: None,
            model: None,
            orientation: None,
        };
    }
}
