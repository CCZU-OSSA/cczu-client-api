use reqwest::Url;

use crate::{client::UserClient, cookies_io::CookiesIOExt};

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
    pub async fn login(&self) {
        let api = format!("{}/web_cas/web_cas_login_jwgl.aspx", self.root);
        self.client.initialize_url(&api);
        let reqwest_client = self.client.get_client();
        if let Ok(response) = reqwest_client.get(api).send().await {
            let redirect_url = response
                .headers()
                .get("location")
                .unwrap()
                .to_str()
                .unwrap();
            self.client.initialize_url(redirect_url);
            if let Ok(response) = reqwest_client.get(redirect_url).send().await {
                let redirect_url = response
                    .headers()
                    .get("location")
                    .unwrap()
                    .to_str()
                    .unwrap();
                self.client.initialize_url(redirect_url);
                if let Ok(_) = reqwest_client.get(redirect_url).send().await {
                    // may handle something later here
                }
            }
        }
    }
    pub async fn get_class_list(&self) {
        let api = format!("{}/web_jxrw/cx_kb_xsgrkb.aspx", self.root);
        self.client.initialize_url(&api);
        self.client
            .get_cookies()
            .lock()
            .unwrap()
            .debug_url_cookies(&Url::parse(&api).unwrap());
        let reqwest_client = self.client.get_client();
        if let Ok(response) = reqwest_client.get(api).send().await {
            println!("{}", response.text().await.unwrap());
        }
    }

    pub async fn get_grades_list(&self) {
        let api = format!("{}/web_cjgl/cx_cj_xh.aspx", self.root);
        self.client.initialize_url(api.as_str());
    }
}
