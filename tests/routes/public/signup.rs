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

    let email = user_data.get("email").unwrap().as_str().unwrap();
    let password = user_data.get("password").unwrap().as_str().unwrap();
    let name = user_data.get("name").unwrap().as_str().unwrap();
    assert_eq!(user.email, email);
    assert!(Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .is_ok());
    assert_eq!(user.name, name);

    Ok(())
}
