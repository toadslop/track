use actix_web_httpauth::headers::authorization::Basic;
use utilities::spawn::spawn_app;

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

// it('Can get the information of test user account (Taro Yamada).', (done) => {
//     chai.request(BASE_URL)
//       .get(GET_USER_PATH + reservedUser.user_id)
//       .auth(reservedUser.user_id, reservedUser.password)
//       .end((err, res) => {
//         res.should.have.status(200);
//         res.body.should.be.a('object');
//         res.body.should.have.property('message').eql('User details by user_id');
//         done();
//       });
//   });
