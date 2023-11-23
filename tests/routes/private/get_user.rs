use actix_web_httpauth::headers::authorization::Basic;
use utilities::{dummy::gen_dummy_user, spawn::spawn_app};

static RESERVED_USER_ID: &str = "TaroYamada";
static RESERVED_USR_PASS: &str = "PaSSwd4TY";

#[actix_web::test]
async fn cannot_get_user_information_without_authorization() -> anyhow::Result<()> {
    // Arrange

    let test_app = spawn_app().await?;
    let expected_code = 401;

    // Act
    let resp = test_app.get_user(RESERVED_USER_ID, None).await?;
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
async fn can_get_information_of_test_user() -> anyhow::Result<()> {
    // Arrange

    let test_app = spawn_app().await?;
    let expected_code = 200;

    // Act
    let resp = test_app
        .get_user(
            RESERVED_USER_ID,
            Some(Basic::new(RESERVED_USER_ID, Some(RESERVED_USR_PASS))),
        )
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

    assert_eq!(message, "User details by user_id");

    Ok(())
}

#[actix_web::test]
async fn can_get_user_information_of_different_user_id() -> anyhow::Result<()> {
    // Arrange

    let test_app = spawn_app().await?;
    let expected_code = 200;
    let user_data = gen_dummy_user();
    test_app.signup(user_data.clone()).await?;
    let user_id = user_data.get("user_id").unwrap().as_str().unwrap();

    // Act
    let resp = test_app
        .get_user(
            user_id,
            Some(Basic::new(RESERVED_USER_ID, Some(RESERVED_USR_PASS))),
        )
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

    assert_eq!(message, "User details by user_id");

    Ok(())
}
