use std::fmt::Display;

use reqwest::StatusCode;

use crate::client::UserClient;

use super::app::Application;
pub struct JwcasApplication<'a> {
    client: &'a mut dyn UserClient,
    root: String,
}

impl<'a> Application<'a> for JwcasApplication<'a> {
    fn from_client(client: &'a mut dyn UserClient) -> Self {
        let root = client.redirect("http://219.230.159.132");
        Self { client, root }
    }
}

impl<'a> JwcasApplication<'a> {
    pub async fn login(&self) -> Result<(), String> {
        let api = format!("{}/web_cas/web_cas_login_jwgl.aspx", self.root);
        self.client.initialize_url(&api);
        let reqwest_client = self.client.get_client();
        if let Ok(response) = reqwest_client.get(api).send().await {
            let redirect_url = response
                .headers()
                .get("location")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            self.client.initialize_url(&redirect_url);
            if let Ok(response) = reqwest_client.get(redirect_url).send().await {
                if response.status() != StatusCode::FOUND {
                    return Err("账户认证失败，请检查登录".into());
                }
                let redirect_url = response
                    .headers()
                    .get("location")
                    .unwrap()
                    .to_str()
                    .unwrap();
                self.client.initialize_url(redirect_url);
                let _ = reqwest_client.get(redirect_url).send().await;
            }
        }
        Ok(())
    }
    pub async fn get_classlist_html(&self) -> Option<String> {
        self.get_api_html("/web_jxrw/cx_kb_xsgrkb.aspx").await
    }

    pub async fn get_gradelist_html(&self) -> Option<String> {
        self.get_api_html("/web_cjgl/cx_cj_xh.aspx").await
    }

    pub async fn get_api_html(&self, service: impl Display) -> Option<String> {
        let api = format!("{}{}", self.root, service);
        self.client.initialize_url(&api);
        if let Ok(response) = self.client.get_client().get(api).send().await {
            Some(response.text().await.unwrap())
        } else {
            None
        }
    }
}
