use std::{collections::HashMap, sync::Arc};

use base64::{prelude::BASE64_STANDARD, Engine};
use reqwest::{Client, Response, Url};
use reqwest_cookie_store::CookieStoreMutex;
use scraper::{Html, Selector};

use crate::{
    cookies_io::CookiesIOExt,
    creq,
    fields::{DEFAULT_HEADERS, ROOT_SSO},
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
    let mut cookies: HashMap<String, String> = HashMap::new();
    if let Ok(response) = creq::Request::get(url.clone()).send().await {
        let text = response.resp.text().await;
        if let Ok(text) = text {
            dom = text;
        }
        for ck in response.cookies {
            cookies.insert(ck.0, ck.1);
        }
        /*
        cookies.lock().unwrap().copy_cookies(
            &url.parse::<Url>().unwrap(),
            &ROOT_SSO.parse::<Url>().unwrap(),
        );
        */
    }

    if dom.is_empty() {
        return Err("SSO 登录失败(无法访问)，请尝试普通登录...".into());
    }

    let mut login_param = parse_hidden_values(dom.as_str());
    login_param.insert("username".into(), user.into());
    login_param.insert("password".into(), BASE64_STANDARD.encode(pwd.into()));

    let mut req = creq::Request::post(format!("{}/sso/login", ROOT_SSO));

    if let Ok(response) = req.forms(login_param.clone()).cookies(cookies).send().await {
        return Ok(response.resp);
    }

    Err("SSO 登录失败".into())
}
