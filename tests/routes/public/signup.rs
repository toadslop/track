use serde_json::json;
use utilities::dummy::gen_dummy_user;
use utilities::spawn::spawn_app;

#[actix_web::test]
async fn cannot_sign_up_account_without_user_id_and_password() -> anyhow::Result<()> {
    // Arrange

    let test_app = spawn_app().await?;

    // Act
    let resp = test_app.signup(&json!({})).await?;
    let status = resp.status();
    let body = resp
        .json::<serde_json::Value>()
        .await
        .expect("Expected a valid json body");

    // Assert
    assert_eq!(
        400,
        status.as_u16(),
        "Expected the api to return 400 but instead got {}",
        status.as_str()
    );

    assert!(body.is_object());
    let message = match body.get("message").as_ref().unwrap() {
        serde_json::Value::String(value) => value,
        _ => panic!("Should have gotten a string"),
    };

    assert_eq!(message, "Account creation failed");

    let message = match body.get("cause").as_ref().unwrap() {
        serde_json::Value::String(value) => value,
        _ => panic!("Should have gotten a string"),
    };

    assert!(message.contains("required"));

    Ok(())
}

#[actix_web::test]
async fn can_create_an_account() -> anyhow::Result<()> {
    // Arrange

    let test_app = spawn_app().await?;
    let user_data = gen_dummy_user();

    // Act
    let resp = test_app.signup(&user_data).await?;
    let status = resp.status();
    let body = resp
        .json::<serde_json::Value>()
        .await
        .expect("Expected a valid json body");

    // Assert
    assert_eq!(
        200,
        status.as_u16(),
        "Expected the api to return {} but instead got {}",
        200,
        status.as_str()
    );

    assert!(body.is_object());
    let message = match body.get("message").as_ref().unwrap() {
        serde_json::Value::String(value) => value,
        _ => panic!("Should have gotten a string"),
    };

    assert_eq!(message, "Account successfully created");

    let user = body.get("user").unwrap();
    let id = user.get("user_id").unwrap();
    let nickname = user.get("nickname").unwrap();
    let submitted_id = user_data.get("user_id").unwrap();

    assert_eq!(id, submitted_id);
    assert_eq!(nickname, submitted_id);

    Ok(())
}
