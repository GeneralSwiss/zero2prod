use crate::helpers::spawn_service;

#[tokio::test]
async fn health_check_works() {
    let test_app = spawn_service().await;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health", test_app.address))
        .send()
        .await
        .expect("Failed to call the health check.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
