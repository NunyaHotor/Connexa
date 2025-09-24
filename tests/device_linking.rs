use reqwest::Client;
use serde_json::json;

#[tokio::test]
async fn test_device_linking_flow() {
    // This test assumes the server is running locally with ALLOW_INSECURE_AUTH=1 and HTTP_ADDR=127.0.0.1:3000
    let client = Client::new();
    let jwt = "test-user"; // In insecure mode, any token (or none) maps to a user id

    // 1. Initiate link
    let resp = client
        .post("http://127.0.0.1:3000/device/link/initiate")
        .bearer_auth(jwt)
        .json(&json!({ "device_name": "Test Device" }))
        .send()
        .await
        .expect("Failed to initiate link");
    assert!(resp.status().is_success());
    let link_token: String = resp.json::<serde_json::Value>().await.unwrap()["link_token"].as_str().unwrap().to_string();

    // 2. Complete link with token
    let complete_resp = client
        .post("http://127.0.0.1:3000/device/link/complete")
        .json(&json!({ "link_token": link_token, "device_name": "Test Device" }))
        .send()
        .await
        .expect("Failed to complete link");
    assert!(complete_resp.status().is_success());
    let device = complete_resp.json::<serde_json::Value>().await.unwrap();
    assert_eq!(device["verified"], true);

    // 3. List devices and assert new device is present and verified
    let list_resp = client
        .get("http://127.0.0.1:3000/devices")
        .bearer_auth(jwt)
        .send()
        .await
        .expect("Failed to list devices");
    assert!(list_resp.status().is_success());
    let devices = list_resp.json::<serde_json::Value>().await.unwrap();
    let found = devices.as_array().unwrap().iter().any(|d| d["verified"] == true && d["name"] == "Test Device");
    assert!(found, "Linked device not found or not verified");
}