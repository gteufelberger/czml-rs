use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
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

    /// A plain text or HTML description of the packet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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
