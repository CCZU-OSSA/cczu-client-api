use std::{cell::RefCell, sync::Arc};

use serde_json::json;

use crate::base::{app::Application, client::AuthClient};

use super::jwqywx_type::{LoginUserData, Message};
#[derive(Clone)]
pub struct JwqywxApplication {
    client: Arc<dyn AuthClient>,
    token: RefCell<String>,
}

impl Application for JwqywxApplication {
    fn from_client(client: Arc<dyn AuthClient>) -> Self {
        Self {
            client,
            token: RefCell::default(),
        }
    }
}

impl JwqywxApplication {
    // Return wx token
    pub async fn login(&self) -> Option<String> {
        let response = self
            .client
            .clone()
            .get_client()
            .post("url")
            .json(&json!({
                "userid":self.client.get_user(),
                "userpwd":self.client.get_pwd()
            }))
            .send()
            .await;
        // TODO Handle Panic here
        let message: Message<LoginUserData> =
            serde_json::from_str(response.unwrap().text().await.unwrap().as_str()).unwrap();
        *self.token.borrow_mut() = message.token.clone();

        Some(message.token)
    }
}
