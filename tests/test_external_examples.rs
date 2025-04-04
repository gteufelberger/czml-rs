use czml::Packet;
use reqwest;
use serde_json;

#[test]
fn test_deserialize_multipart_vehicle_example_from_github() {
    let urls = [
        "https://raw.githubusercontent.com/CesiumGS/cesium/refs/heads/main/Apps/SampleData/MultipartVehicle_part1.czml",
        "https://raw.githubusercontent.com/CesiumGS/cesium/refs/heads/main/Apps/SampleData/MultipartVehicle_part2.czml",
        "https://raw.githubusercontent.com/CesiumGS/cesium/refs/heads/main/Apps/SampleData/MultipartVehicle_part3.czml",
    ];

    for url in urls {
        // Download the CZML file
        let response = reqwest::blocking::get(url).expect("Failed to fetch CZML file");
        assert!(response.status().is_success(), "Failed to fetch CZML file");

        let czml_text = response.text().expect("Failed to read response text");

        // Deserialize
        let packets: Vec<Packet> =
            serde_json::from_str(&czml_text).expect("Failed to deserialize CZML");

        println!("{:#?}", packets);
    }
}

#[test]
fn test_deserialize_clamp_to_ground_example_from_github() {
    let url =
        "https://raw.githubusercontent.com/CesiumGS/cesium/refs/heads/main/Apps/SampleData/ClampToGround.czml";

    // Download the CZML file
    let response = reqwest::blocking::get(url).expect("Failed to fetch CZML file");
    assert!(response.status().is_success(), "Failed to fetch CZML file");

    let czml_text = response.text().expect("Failed to read response text");

    // Deserialize
    let packets: Vec<Packet> =
        serde_json::from_str(&czml_text).expect("Failed to deserialize CZML");

    println!("{:#?}", packets);
}

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
