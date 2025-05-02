use std::fs;
use std::path::Path;

#[test]
fn test_deserialize_cylinder() {
    let path = Path::new("tests/data/cylinder_field.czml");
    let json_str = fs::read_to_string(path).expect("Failed to read JSON file");
    let expected: Vec<czml::Packet> =
        serde_json::from_str(&json_str).expect("Failed to parse JSON");

    println!("{:#?}", expected);
}
