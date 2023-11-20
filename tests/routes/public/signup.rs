use serde_json::json;
use track_api_challenge::domain::user::User;
use utilities::spawn::spawn_app;

#[actix_web::test]
async fn signup_returns_200_for_valid_data() -> anyhow::Result<()> {
    // Arrange
    let email = "test@test.com";
    let name = "tom";
    let password = "password";
    let test_app = spawn_app().await?;
    let user_data = json!({
        "email": email,
        "name": name,
        "password": password
    });

    // Act
    let resp = test_app.signup(&user_data).await?;

    // Assert
    assert_eq!(
        200,
        resp.status().as_u16(),
        "Expected the api to return 200 but instead got {}",
        resp.status().as_str()
    );

    let user = resp.json::<User>().await?;

    assert_eq!(user.email, email);
    assert_eq!(user.password, password);
    assert_eq!(user.name, name);

    Ok(())
}
