use utilities::spawn::spawn_app;

#[actix_web::test]
async fn health_check_returns_200() -> anyhow::Result<()> {
    // Arrange
    let test_app = spawn_app().await?;

    // Act
    let resp = test_app.health_check().await;

    // Assert
    assert_eq!(
        200,
        resp.status().as_u16(),
        "Expected the api to return 200 but instead got {}",
        resp.status().as_str()
    );

    Ok(())
}
