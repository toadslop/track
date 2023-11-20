use fake::faker::internet::en::Password;
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use fake::Fake;
use serde_json::json;

pub fn gen_dummy_user() -> serde_json::Value {
    let email: String = SafeEmail().fake();
    let name: String = Name().fake();
    let password: String = Password(8..16).fake();

    json!({
        "email": email,
        "name": name,
        "password": password
    })
}
