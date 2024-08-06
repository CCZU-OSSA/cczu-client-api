use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    base::{app::Application, client::AuthClient},
    internals::fields::{DEFAULT_HEADERS, WECHAT_APP_API},
};

use super::jwqywx_type::{CourseGrade, LoginUserData, Message, StudentPoint};
#[derive(Clone)]
pub struct JwqywxApplication {
    client: Arc<dyn AuthClient + Send + Sync>,
    token: Arc<RwLock<String>>,
}

impl Application for JwqywxApplication {
    fn from_client(client: Arc<dyn AuthClient + Send + Sync>) -> Self {
        Self {
            client,
            token: Arc::new(RwLock::new(String::new())),
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
                let message = serde_json::from_str::<Message<LoginUserData>>(&text).unwrap();
                {
                    *self.token.write().await =
                        format!("Bearer {}", message.token.clone().unwrap());
                    return Some(message);
                }
            }
        }

        None
    }

    async fn headers(&self) -> HeaderMap {
        let mut header = DEFAULT_HEADERS.clone();
        header.insert(
            "Authorization",
            HeaderValue::from_str(self.token.read().await.clone().as_str()).unwrap(),
        );
        header.insert(
            "Referer",
            HeaderValue::from_static("http://jwqywx.cczu.edu.cn/"),
        );
        header.insert(
            "Origin",
            HeaderValue::from_static("http://jwqywx.cczu.edu.cn"),
        );
        header
    }

    pub async fn get_grades(&self) -> Option<Message<CourseGrade>> {
        let result = self
            .client
            .clone()
            .get_client()
            .post(format!("{}/api/cj_xh", WECHAT_APP_API))
            .headers(self.headers().await)
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

    pub async fn get_points(&self) -> Option<Message<StudentPoint>> {
        let result = self
            .client
            .clone()
            .get_client()
            .post(format!("{}/api/cj_xh_xfjdpm", WECHAT_APP_API))
            .headers(self.headers().await)
            .json(&json!({
                "xh":self.client.get_user(),
            }))
            .send()
            .await;
        if let Ok(response) = result {
            let message = response.json::<Message<StudentPoint>>().await.unwrap();

            return Some(message);
        }
        None
    }
}
