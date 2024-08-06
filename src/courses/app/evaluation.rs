use std::sync::Arc;

use crate::{
    base::{app::Application, client::AuthClient},
    internals::fields::DEFAULT_HEADERS,
};

const EVALUTION_BASE_URL: &'static str = "http://202.195.102.53";

pub struct EvalutionApplication {
    client: Arc<dyn AuthClient + Send + Sync>,
}
impl Application for EvalutionApplication {
    fn from_client(client: Arc<dyn AuthClient + Send + Sync>) -> Self {
        Self { client }
    }
}

impl EvalutionApplication {
    pub async fn login(&self) {
        self.client
            .get_client()
            .post(EVALUTION_BASE_URL)
            .headers(DEFAULT_HEADERS.clone())
            .send()
            .await
            .unwrap();
        todo!()
    }
}
