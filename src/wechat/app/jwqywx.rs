use std::{cell::RefCell, sync::Arc};

use serde_json::json;

use crate::{
    base::{app::Application, client::AuthClient},
    internals::fields::{DEFAULT_HEADERS, WECHAT_APP_API},
};

use super::jwqywx_type::{CourseGrade, LoginUserData, Message};
#[derive(Clone)]
pub struct JwqywxApplication {
    client: Arc<dyn AuthClient + Send + Sync>,
    token: RefCell<String>,
}

impl Application for JwqywxApplication {
    fn from_client(client: Arc<dyn AuthClient + Send + Sync>) -> Self {
        Self {
            client,
            token: RefCell::default(),
        }
    }
}

impl JwqywxApplication {
    pub async fn login(&self) -> Option<Message<LoginUserData>> {
        let result = self
            .client
            .clone()
            .get_client()
            .post(format!("{}/api/login", WECHAT_APP_API))
            .headers(DEFAULT_HEADERS.clone())
            .header("Referer", "http://jwqywx.cczu.edu.cn/")
            .header("Origin", "http://jwqywx.cczu.edu.cn")
            .json(&json!({
                "userid":self.client.get_user(),
                "userpwd":self.client.get_pwd()
            }))
            .send()
            .await;
        if let Ok(response) = result {
            if let Ok(text) = response.text().await {
                println!("{}", text);
                let message = serde_json::from_str::<Message<LoginUserData>>(&text).unwrap();
                {
                    *self.token.borrow_mut() = format!("Bearer {}", message.token);
                    return Some(message);
                }
            }
        }

        None
    }

    pub async fn get_grades(&self) -> Option<Message<CourseGrade>> {
        let result = self
            .client
            .clone()
            .get_client()
            .post(format!("{}/api/cj_xh", WECHAT_APP_API))
            .headers(DEFAULT_HEADERS.clone())
            .header("Authorization", self.token.borrow().clone())
            .header("Referer", "http://jwqywx.cczu.edu.cn/")
            .header("Origin", "http://jwqywx.cczu.edu.cn")
            .json(&json!({
                "xh":self.client.get_user(),
            }))
            .send()
            .await;
        if let Ok(response) = result {
            let message = response.json::<Message<CourseGrade>>().await.unwrap();

            return Some(message);
        }
        None
    }
}
