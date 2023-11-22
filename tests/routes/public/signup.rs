use argon2::{Argon2, PasswordHash, PasswordVerifier};
use track_api_challenge::domain::user::User;
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

    let user = resp.json::<User>().await?;
    let hash = PasswordHash::new(&user.password).unwrap();

    let user_id = user_data.get("user_id").unwrap().as_str().unwrap();
    let password = user_data.get("password").unwrap().as_str().unwrap();

    assert_eq!(user.user_id, user_id);
    assert!(Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .is_ok());

    Ok(())
}
