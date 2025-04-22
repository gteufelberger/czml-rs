use std::fs;
use std::path::Path;

#[test]
fn test_deserialize_example1() {
    // https://czml3.readthedocs.io/en/v2.3.4/user/examples.html#example-1
    let path = Path::new("tests/data/example1.czml");
    let json_str = fs::read_to_string(path).expect("Failed to read JSON file");
    let expected: Vec<czml::Packet> =
        serde_json::from_str(&json_str).expect("Failed to parse JSON");

    println!("{:#?}", expected);
}
