use std::{collections::HashMap, sync::Arc};

use base64::{prelude::BASE64_STANDARD, Engine};
use reqwest::{redirect::Policy, Client, ClientBuilder, Response, StatusCode, Url};
use reqwest_cookie_store::CookieStoreMutex;
use scraper::{Html, Selector};

use crate::{
    cookies_io::CookiesIOExt,
    fields::{ROOT_SSO, ROOT_SSO_LOGIN, ROOT_SSO_LOGIN_URL, ROOT_VPN_URL},
    recursion::recursion_cookies_handle,
};

pub fn parse_hidden_values(html: &str) -> HashMap<String, String> {
    let mut hidden_values = HashMap::new();
    let dom = Html::parse_document(html);
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

pub async fn sso_login<S>(
    client: Arc<Client>,
    cookies: Arc<CookieStoreMutex>,
    user: S,
    pwd: S,
    service: S,
) -> Result<Response, String>
where
    S: Into<String>,
{
    let mut dom = String::new();
    let url: String = format!("{}/sso/login?service={}", ROOT_SSO, service.into());
    if let Ok(response) = client.get(url.clone()).send().await {
        let text = response.text().await;
        if let Ok(text) = text {
            dom = text;
        }

        cookies
            .lock()
            .unwrap()
            .copy_cookies(&url.parse::<Url>().unwrap(), &ROOT_SSO_LOGIN_URL);
    }

    if dom.is_empty() {
        return Err("SSO 登录失败(无法访问)，请尝试普通登录...".into());
    }

    let mut login_param = parse_hidden_values(dom.as_str());
    login_param.insert("username".into(), user.into());
    login_param.insert("password".into(), BASE64_STANDARD.encode(pwd.into()));

    if let Ok(response) = client.post(ROOT_SSO_LOGIN).form(&login_param).send().await {
        return Ok(response);
    };

    Err("SSO 登录失败".into())
}

#[tokio::test]
pub async fn test_universal_sso_login() {
    let cookies = Arc::new(CookieStoreMutex::default());
    let client = ClientBuilder::new()
        .cookie_provider(cookies.clone())
        .redirect(Policy::none())
        .build()
        .unwrap();
    if let Ok(response) = client.get(ROOT_SSO_LOGIN).send().await {
        // use webvpn
        if response.status() == StatusCode::FOUND {
            // redirect to webvpn root

            recursion_cookies_handle(
                client,
                cookies.clone(),
                response
                    .headers()
                    .get("location")
                    .unwrap()
                    .to_str()
                    .unwrap(),
                &ROOT_VPN_URL,
            )
            .await
            .unwrap();
            cookies.lock().unwrap().debug_url_cookies(&ROOT_VPN_URL);
        }
        // connect cczu
        if response.status() == StatusCode::OK {}
    }
}

pub async fn is_webvpn_available() -> bool {
    let client = ClientBuilder::new()
        .redirect(Policy::none())
        .build()
        .unwrap();
    if let Ok(response) = client.get(ROOT_SSO_LOGIN).send().await {
        return response.status() == StatusCode::FOUND;
    }
    false
}
