use crate::fields::{DEFAULT_HEADERS, ROOT_SSO, ROOT_VPN};
use base64::{prelude::BASE64_STANDARD, Engine};
use reqwest::{
    cookie::{Cookie, Jar},
    Client,
};
use scraper::{Html, Selector};
use std::{collections::HashMap, fmt::format, sync::Arc};

pub struct WebVpnClient {
    pub user: String,
    pub pwd: String,
    pub client: Client,
    cookies: Arc<Jar>,
}

fn parse_hidden_values(html: String) -> HashMap<String, String> {
    let mut hidden_values = HashMap::new();
    let dom = Html::parse_document(&*html);
    let input_hidden_selector = Selector::parse(r#"input[type="hidden"]"#).unwrap();
    let tags_hidden = dom.select(&input_hidden_selector);

    tags_hidden.for_each(|tag_hidden| {
        hidden_values.insert(
            tag_hidden.attr("name").unwrap().to_string(),
            tag_hidden.attr("value").unwrap().to_string(),
        );
    });

    hidden_values
}

impl WebVpnClient {
    pub fn new(user: &str, pwd: &str) -> Self {
        let cookies = Arc::new(Jar::default());
        WebVpnClient {
            user: user.to_string(),
            pwd: pwd.to_string(),
            client: Client::builder()
                .cookie_provider(cookies.clone())
                .build()
                .unwrap(),
            cookies: cookies.clone(),
        }
    }

    /**
     * SSO 登录
     */
    pub async fn sso_login(&self) -> Result<String, String> {
        let mut j_session_id = String::new();
        let mut dom = String::new();

        let url = format!(
            "{}/sso/login?service={}/enlink/api/client/callback/cas",
            ROOT_SSO, ROOT_VPN
        );

        if let Ok(resp) = self
            .client
            .get(url)
            .headers(DEFAULT_HEADERS.clone())
            .send()
            .await
        {
            if let Some(cookie) = &resp
                .cookies()
                .filter(|cookie| cookie.name() == "JSESSIONID")
                .collect::<Vec<Cookie>>()
                .first()
            {
                j_session_id = cookie.value().into();
            }

            if let Ok(text) = &resp.text().await {
                dom = text.into();
            }
        }

        if j_session_id.is_empty() || dom.is_empty() {
            return Err("Sso登录失败(无法访问)，请尝试普通登录...".into());
        }

        let mut login_param = parse_hidden_values(dom);
        login_param.insert("username".into(), self.user.clone());
        login_param.insert("password".into(), BASE64_STANDARD.encode(self.pwd.clone()));
        if let Ok(resp) = self
            .client
            .post(format!(
                "{}/sso/login;jsessionid={}",
                ROOT_SSO, j_session_id
            ))
            .headers(DEFAULT_HEADERS.clone())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&login_param)
            .send()
            .await
        {
            // Cookie...
        }

        Err("Sso登录失败，请尝试普通登录...".into())
    }
}
