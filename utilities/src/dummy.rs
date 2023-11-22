use fake::faker::internet::en::Password;
use fake::faker::internet::en::Username;
use fake::Fake;
use serde_json::json;

pub fn gen_dummy_user() -> serde_json::Value {
    let name: String = Username().fake();
    let password: String = Password(8..16).fake();

    json!({
        "user_id": name,
        "password": password
    })
}
