use actix_web_httpauth::headers::authorization::Basic;
use fake::faker::company::en::BsAdj;
use fake::faker::company::en::BsNoun;
use fake::faker::company::en::CatchPhase;
use fake::Fake;
use serde_json::json;
use utilities::{dummy::gen_dummy_user, spawn::spawn_app};

#[actix_web::test]
async fn cannot_update_user_information_without_authorization() -> anyhow::Result<()> {
    // Arrange

    let test_app = spawn_app().await?;
    let expected_code = 401;
    let user_data = gen_dummy_user();
    test_app.signup(&user_data).await?;
    let user_id = user_data.get("user_id").unwrap().to_string();

    let new_details = json!({
        "nickname": format!("{} {}", BsAdj().fake::<String>(), BsNoun().fake::<String>()),
        "comment": CatchPhase().fake::<String>()
    });

    // Act
    let resp = test_app.update_user(&user_id, None, &new_details).await?;

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
async fn can_update_user_information() -> anyhow::Result<()> {
    // Arrange
    let test_app = spawn_app().await?;
    let expected_code = 200;
    let user_data = gen_dummy_user();
    test_app.signup(&user_data).await?;
    let user_id = user_data.get("user_id").unwrap().to_string();
    let password = user_data.get("password").unwrap().to_string();
    let fixed_id = &user_id[1..(user_id.len() - 1)];
    let fixed_password = &password[1..(password.len() - 1)];

    let mut nickname = format!("{} {}", BsAdj().fake::<String>(), BsNoun().fake::<String>());
    nickname.truncate(20);
    let mut comment = CatchPhase().fake::<String>();
    comment.truncate(30);

    let new_details = json!({
        "nickname": nickname,
        "comment": comment
    });

    // Act
    let resp = test_app
        .update_user(
            fixed_id,
            Some(Basic::new(
                fixed_id.to_owned(),
                Some(fixed_password.to_owned()),
            )),
            &new_details,
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

    assert_eq!(message, "User successfully updated");

    Ok(())
}
