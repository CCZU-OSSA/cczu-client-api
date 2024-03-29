use reqwest::{StatusCode, Url};

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
    pub fn login(&self) {
        let _api = format!("{}/web_cas/web_cas_login_jwgl.aspx", self.root);
    }
    pub async fn get_class_list(&self) {
        let api = format!("{}/web_jxrw/cx_kb_xsgrkb.aspx", self.root);
        self.client.initialize_url(api.as_str());
        let reqwest_client = self.client.get_client();
        if let Ok(response) = reqwest_client.get(Url::parse(&api).unwrap()).send().await {
            if response.status() == StatusCode::FOUND {
                if let Ok(_response) = reqwest_client
                    .get(
                        response
                            .headers()
                            .get("location")
                            .unwrap()
                            .to_str()
                            .unwrap(),
                    )
                    .send()
                    .await
                {}
            }
        }
    }

    pub async fn get_grades_list(&self) {
        let api = format!("{}/web_cjgl/cx_cj_xh.aspx", self.root);
        self.client.initialize_url(api.as_str());
    }
}
