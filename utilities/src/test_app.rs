use crate::dummy::gen_dummy_user;
use base64::engine::general_purpose;
use base64::Engine;
use reqwest::RequestBuilder;
use serde_json;
use track_api_challenge::actix_web_httpauth::headers::authorization::Basic;
use track_api_challenge::anyhow;
use track_api_challenge::database::Database;

pub struct TestApp {
    app_address: reqwest::Url,
    client: reqwest::Client,
    db: Database,
}

impl TestApp {
    pub fn new(app_address: reqwest::Url, db: Database) -> Self {
        Self {
            app_address,
            client: reqwest::Client::new(),
            db,
        }
    }

    pub fn app_address(&self) -> &reqwest::Url {
        &self.app_address
    }
}

impl TestApp {
    pub async fn health_check(&self) -> anyhow::Result<reqwest::Response> {
        let res = self
            .client
            .get(self.app_address.join("/health_check")?)
            .send()
            .await?;

        Ok(res)
    }

    pub async fn signup(&self, data: &serde_json::Value) -> anyhow::Result<reqwest::Response> {
        let res = self
            .client
            .post(self.app_address.join("/signup")?)
            .json(data)
            .send()
            .await?;

        Ok(res)
    }

    pub async fn signin(&self, data: &serde_json::Value) -> anyhow::Result<reqwest::Response> {
        let res = self
            .client
            .post(self.app_address.join("/signin")?)
            .json(data)
            .send()
            .await?;

        Ok(res)
    }

    pub async fn get_user(
        &self,
        user_id: &str,
        credentials: Option<Basic>,
    ) -> anyhow::Result<reqwest::Response> {
        let mut req = self
            .client
            .get(self.app_address.join(&format!("/users/{user_id}"))?);

        if let Some(credentials) = credentials {
            req = Self::add_auth(req, credentials);
        }

        let res = req.send().await?;

        Ok(res)
    }

    fn add_auth(req: RequestBuilder, credentials: Basic) -> RequestBuilder {
        let raw = format!(
            "{}:{}",
            credentials.user_id(),
            credentials.password().unwrap()
        );
        let mut buf = String::new();
        general_purpose::STANDARD.encode_string(raw, &mut buf);
        req.header("Authorization", format!("Basic {buf}"))
    }

    pub async fn update_user(
        &self,
        user_id: &str,
        credentials: Option<Basic>,
        user_info: &serde_json::Value,
    ) -> anyhow::Result<reqwest::Response> {
        let mut req = self
            .client
            .patch(self.app_address.join(&format!("/users/{user_id}"))?);

        if let Some(credentials) = credentials {
            req = Self::add_auth(req, credentials);
        }

        let res = req.json(user_info).send().await?;

        Ok(res)
    }

    pub async fn base_url(&self) -> anyhow::Result<reqwest::Response> {
        let res = self.client.post(self.app_address.join("/")?).send().await?;

        Ok(res)
    }

    pub async fn create_and_signup_user(&self) -> anyhow::Result<reqwest::Response> {
        let user_data = gen_dummy_user();
        Ok(self.signup(&user_data).await?)
    }

    pub fn db(&mut self) -> &mut Database {
        &mut self.db
    }
}
