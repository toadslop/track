use anyhow::Ok;
use serde_json;
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
    const PUBLIC: &'static str = "/public";

    pub async fn health_check(&self) -> anyhow::Result<reqwest::Response> {
        let res = self
            .client
            .get(
                self.app_address
                    .join(format!("{}/health_check", Self::PUBLIC).as_str())?,
            )
            .send()
            .await?;

        Ok(res)
    }

    pub async fn signup(&self, data: &serde_json::Value) -> anyhow::Result<reqwest::Response> {
        let res = self
            .client
            .post(
                self.app_address
                    .join(format!("{}/signup", Self::PUBLIC).as_str())?,
            )
            .json(data)
            .send()
            .await?;

        Ok(res)
    }

    pub fn db(&mut self) -> &mut Database {
        &mut self.db
    }
}
