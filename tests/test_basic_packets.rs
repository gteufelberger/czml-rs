use std::fs;
use std::path::Path;

#[test]
fn test_just_document_packet() {
    let path = Path::new("tests/data/minimal_document.czml");
    let json_str = fs::read_to_string(path).expect("Failed to read JSON file");
    let expected: Vec<czml::Packet> =
        serde_json::from_str(&json_str).expect("Failed to parse JSON");

    let actual = czml::Packet {
        id: "document".to_string(),
        name: None,
        version: Some("1.0".to_string()),
        availability: None,
        description: None,
        billboard: None,
    };

    assert_eq!(actual, expected[0]);
}
