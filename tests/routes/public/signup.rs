use serde_json::json;
use utilities::dummy::gen_dummy_user;
use utilities::spawn::spawn_app;

#[actix_web::test]
async fn signup_returns_200_for_valid_data() -> anyhow::Result<()> {
    // Arrange

    let test_app = spawn_app().await?;
    let user_data = gen_dummy_user();

    // Act
    let resp = test_app.signup(user_data.clone()).await?;

    // Assert
    assert_eq!(
        200,
        resp.status().as_u16(),
        "Expected the api to return 200 but instead got {}",
        resp.status().as_str()
    );

    // let user = resp.json::<User>().await?;
    // let hash = PasswordHash::new(&user.password).unwrap();

    // let user_id = user_data.get("user_id").unwrap().as_str().unwrap();
    // let password = user_data.get("password").unwrap().as_str().unwrap();

    // assert_eq!(user.user_id, user_id);
    // assert!(Argon2::default()
    //     .verify_password(password.as_bytes(), &hash)
    //     .is_ok());

    Ok(())
}

#[actix_web::test]
async fn cannot_sign_up_account_without_user_id_and_password() -> anyhow::Result<()> {
    // Arrange

    let test_app = spawn_app().await?;

    // Act
    let resp = test_app.signup(json!({})).await?;
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
    let resp = test_app.signup(user_data).await?;
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

    Ok(())
}
