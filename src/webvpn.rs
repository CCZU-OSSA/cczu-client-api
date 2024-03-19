use std::collections::HashMap;
use std::sync::Arc;

use base64::alphabet::STANDARD;
use reqwest::Client;
use reqwest::cookie::{CookieStore, Jar};
use scraper::{Html, Selector};
use scraper::selectable::Selectable;

use crate::types::DEFAULT_HEADERS;

pub struct WebVpnClient {
    user: String,
    pwd: String,
    client: Client,
}

impl WebVpnClient {
    const ROOT_SSO: &'static str = "http://sso.cczu.edu.cn";
    const ROOT_VPN: &'static str = "https://zmvpn.cczu.edu.cn";
    const COOKIE_STORE: dyn CookieStore = CookieStore::new(None);


    pub fn new(user: &str, pwd: &str) -> Self {
        WebVpnClient {
            user: user.to_string(),
            pwd: pwd.to_string(),
            client: Client::builder().cookie_provider(Arc::new(Self::COOKIE_STORE)).build().unwrap(),
        }
    }

    /**
     * Sso登录
     */
    pub async fn sso_login(&self) -> Result<String, String> {
        let mut j_session_id = String::new();
        let mut dom = String::new();
        let url = Self::ROOT_SSO.clone().to_string() + "/sso/login?service=" + Self::ROOT_VPN.clone() + "/enlink/api/client/callback/cas";
        if let Ok(resp) = self.client
            .get(url)
            .headers(DEFAULT_HEADERS.clone())
            .send()
            .await
        {
            if let Ok(resp) = resp.text() {
                dom = resp.text();
            }
            if let Ok(cookies) = resp.cookies() {
                for cookie in cookies {
                    if cookie.name() == "JSESSIONID" {
                        j_session_id = cookie.value().to_string();
                    }
                }
            }
        }
        if j_session_id.is_empty() || dom.is_empty() {
            return Err("Sso登录失败(无法访问)，请尝试普通登录...".to_string());
        }
        let mut login_param = Self::parse_hidden_values(dom);
        login_param.insert("username", self.user.clone());
        login_param.insert("password", STANDARD.encode(self.pwd.clone()));
        if let Ok(resp) = self.client
            .post("")
            .headers(DEFAULT_HEADERS.clone())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(login_param)
            .send()
            .await
        {
            // Cookie...
        }
        return Err("Sso登录失败，请尝试普通登录...".to_string());
    }

    fn parse_hidden_values(html: String) -> HashMap<String, String> {
        let mut hidden_values = HashMap::new();
        let dom = Html::parse_document(&*html);
        let input_hidden_selector = Selector::parse(r#"input[type="hidden"]"#);
        let tags_hidden = dom.select(input_hidden_selector);
        for tag_hidden in tags_hidden {
            hidden_values.insert(tag_hidden.attr("name").unwrap(), tag_hidden.attr("value").unwrap());
        }
        return hidden_values;
    }
}
