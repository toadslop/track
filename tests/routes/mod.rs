use crate::util::spawn_app;

#[actix_web::test]
async fn hello_world_always_returns_200() {
    // Arrange
    let test_app = spawn_app().await.expect("Failed to spawn app.");

    // Act
    let resp = test_app.get_hello_world().await;

    // Assert
    assert_eq!(
        200,
        resp.status().as_u16(),
        "The API did not fail with 401 Unauthorized when the requester was not logged in"
    );
}
