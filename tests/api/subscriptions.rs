use crate::helpers::spawn_service;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Given
    let test_app = spawn_service().await;

    // When
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
        let response = test_app.post_subscriptions(body.to_string()).await;

    // Then
    assert_eq!(200, response.status());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Given
    let test_app = spawn_service().await;
    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // When
        let response = test_app.post_subscriptions(invalid_body.to_string()).await;

        // Then
        assert_eq!(
            400,
            response.status(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_empty() {
    let test_app = spawn_service().await;
    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursual&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = test_app.post_subscriptions(body.to_string()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a status 400 Bad Request when payload was {}",
            description
        )
    }
}
