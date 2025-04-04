use czml::Packet;
use reqwest;
use serde_json;

#[test]
fn test_deserialize_vehicle_example_from_github() {
    let url =
        "https://raw.githubusercontent.com/CesiumGS/cesium/refs/heads/main/Apps/SampleData/Vehicle.czml";

    // Download the CZML file
    let response = reqwest::blocking::get(url).expect("Failed to fetch CZML file");
    assert!(response.status().is_success(), "Failed to fetch CZML file");

    let czml_text = response.text().expect("Failed to read response text");

    // Deserialize
    let packets: Vec<Packet> =
        serde_json::from_str(&czml_text).expect("Failed to deserialize CZML");

    println!("{:#?}", packets);
}
