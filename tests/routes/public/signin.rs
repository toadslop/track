use serde_json::json;
use track_api_challenge::domain::user::User;
use utilities::{dummy::gen_dummy_user, spawn::spawn_app};

#[actix_web::test]
async fn signin_returns_200_and_jwt() -> anyhow::Result<()> {
    // Arrange

    let test_app = spawn_app().await?;
    let user_data = gen_dummy_user();
    test_app
        .signup(user_data.clone())
        .await?
        .json::<User>()
        .await?;

    let email = user_data.get("email").unwrap().as_str().unwrap();
    let password = user_data.get("password").unwrap().as_str().unwrap();
    let signin_data = json!({"email": email, "password": password});

    // Act
    let resp = test_app.signin(signin_data).await?;

    // Assert
    assert_eq!(
        200,
        resp.status().as_u16(),
        "Expected the api to return 200 but instead got {}",
        resp.status().as_str()
    );

    let data = resp.json::<serde_json::Value>().await?;

    let token = match &data {
        serde_json::Value::Object(map) => map.get("token").unwrap().as_str().unwrap(),
        _ => panic!("Invalid data received"),
    };

    jsonwebtoken::decode_header(token)?;

    Ok(())
}
