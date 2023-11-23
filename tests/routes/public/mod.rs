use utilities::spawn::spawn_app;

mod health;
mod signup;

#[actix_web::test]
async fn accessing_base_url_returns_404() -> anyhow::Result<()> {
    // Arrange
    let test_app = spawn_app().await?;

    // Act
    let resp = test_app.base_url().await?;

    // Assert
    assert_eq!(
        404,
        resp.status().as_u16(),
        "Expected the api to return 200 but instead got {}",
        resp.status().as_str()
    );

    Ok(())
}
