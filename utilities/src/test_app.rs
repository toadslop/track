pub struct TestApp {
    app_address: reqwest::Url,
    client: reqwest::Client,
}

impl TestApp {
    pub fn new(app_address: reqwest::Url) -> Self {
        Self {
            app_address,
            client: reqwest::Client::new(),
        }
    }

    pub fn app_address(&self) -> &reqwest::Url {
        &self.app_address
    }
}

impl TestApp {
    pub async fn health_check(&self) -> reqwest::Response {
        self.client
            .get(self.app_address.join("/health_check").unwrap())
            .send()
            .await
            .expect("failed to execute request")
    }
}
