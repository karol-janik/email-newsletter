// Inspect what code gets generated using:
// `cargo expand --test health_check`
#[actix_web::test]
async fn health_check_works() {
    // arrange
    spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our app in the background
fn spawn_app() {
    let server = newsletter::run("127.0.0.1:0000").expect("Failed to bind address");
    // Lunch the server as a background task
    let _ = tokio::spawn(server);
}
