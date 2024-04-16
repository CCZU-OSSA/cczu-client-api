use std::{fmt::Display, sync::Arc};

use reqwest::StatusCode;
use scraper::{ElementRef, Html, Selector};

use crate::client::UserClient;

use super::{base::Application, jwcas_type::GradeData};

#[derive(Clone)]
pub struct JwcasApplication {
    client: Arc<dyn UserClient + Send + Sync>,
    root: String,
}

impl Application for JwcasApplication {
    fn from_client(client: Arc<dyn UserClient + Sync + Send>) -> Self {
        let root = client.redirect("http://219.230.159.132");
        Self { client, root }
    }
}

impl JwcasApplication {
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
        self.get_api_html("/web_cjgl/cx_cj_jxjhcj_xh.aspx").await
    }

    pub async fn get_api_html(&self, service: impl Display) -> Option<String> {
        let api = format!("{}{}", self.root, service);
        self.client.initialize_url(&api);
        if let Ok(response) = self.client.get_client().get(api).send().await {
            if response.status() != StatusCode::OK {
                todo!()
            } else {
                Some(response.text().await.unwrap())
            }
        } else {
            None
        }
    }

    pub async fn get_gradeinfo_vec(&self) -> Result<Vec<GradeData>, String> {
        if let Some(text) = self.get_gradelist_html().await {
            let selector = Selector::parse(r#"tr[class="dg1-item"]"#).unwrap();
            let dom = Html::parse_document(&text);
            Ok(dom
                .select(&selector)
                .map(|e| {
                    let childs: Vec<ElementRef> = e.child_elements().collect();
                    dbg!(GradeData {
                        name: extract_string(childs.get(5).unwrap()),
                        point: extract_string(childs.get(8).unwrap()),
                        grade: extract_string(childs.get(9).unwrap()),
                    })
                })
                .collect())
        } else {
            Err("获取页面失败".into())
        }
    }
}
fn extract_string(element: &ElementRef) -> String {
    element.text().next().unwrap().to_string()
}
