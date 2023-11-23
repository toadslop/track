use actix_web_httpauth::headers::authorization::Basic;
use utilities::{dummy::gen_dummy_user, spawn::spawn_app};

#[actix_web::test]
async fn cannot_delete_account_without_authorization() -> anyhow::Result<()> {
    // Arrange

    let test_app = spawn_app().await?;
    let expected_code = 401;
    let user_data = gen_dummy_user();
    test_app.signup(&user_data).await?;

    // Act
    let resp = test_app.close_account(None).await?;

    let status = resp.status();

    let body = resp
        .json::<serde_json::Value>()
        .await
        .expect("Expected a valid json body");

    // Assert
    assert_eq!(
        expected_code,
        status.as_u16(),
        "Expected the api to return {} but instead got {}",
        expected_code,
        status.as_str()
    );

    assert!(body.is_object());
    let message = match body.get("message").as_ref().unwrap() {
        serde_json::Value::String(value) => value,
        _ => panic!("Should have gotten a string"),
    };

    assert_eq!(message, "Authentication Failed");

    Ok(())
}

#[actix_web::test]
async fn can_delete_account() -> anyhow::Result<()> {
    // Arrange

    let test_app = spawn_app().await?;
    let expected_code = 200;
    let user_data = gen_dummy_user();
    test_app.signup(&user_data).await?;
    let user_id = user_data.get("user_id").unwrap().to_string();
    let password = user_data.get("password").unwrap().to_string();
    let fixed_id = &user_id[1..(user_id.len() - 1)];
    let fixed_password = &password[1..(password.len() - 1)];

    // Act
    let resp = test_app
        .close_account(Some(Basic::new(
            fixed_id.to_owned(),
            Some(fixed_password.to_owned()),
        )))
        .await?;

    let status = resp.status();

    let body = resp
        .json::<serde_json::Value>()
        .await
        .expect("Expected a valid json body");

    // Assert
    assert_eq!(
        expected_code,
        status.as_u16(),
        "Expected the api to return {} but instead got {}",
        expected_code,
        status.as_str()
    );

    assert!(body.is_object());
    let message = match body.get("message").as_ref().unwrap() {
        serde_json::Value::String(value) => value,
        _ => panic!("Should have gotten a string"),
    };

    assert_eq!(message, "Account and user successfully removed");

    Ok(())
}
