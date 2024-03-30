use std::{collections::HashMap, sync::Arc};

use base64::{prelude::BASE64_STANDARD, Engine};
use reqwest::{redirect::Policy, Client, ClientBuilder, StatusCode};
use reqwest_cookie_store::CookieStoreMutex;
use scraper::{Html, Selector};

use crate::{
    cookies_io::CookiesIOExt,
    fields::{DEFAULT_HEADERS, ROOT_SSO_LOGIN, ROOT_VPN_URL},
    recursion::recursion_cookies_handle,
    types::{LoginConnectType, UniversalSSOLogin},
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

pub async fn universal_sso_login<S>(
    client: Arc<Client>,
    cookies: Arc<CookieStoreMutex>,
    user: S,
    pwd: S,
) -> Result<UniversalSSOLogin, String>
where
    S: Into<String> + Clone,
{
    if let Ok(response) = client.get(ROOT_SSO_LOGIN).send().await {
        // use webvpn
        if response.status() == StatusCode::FOUND {
            // redirect to webvpn root
            // recursion to get the login page
            if let Ok(response) = recursion_cookies_handle(
                client.clone(),
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
            {
                let url = response.url().clone();
                let dom = response.text().await.unwrap();
                let mut login_param = parse_hidden_values(dom.as_str());
                login_param.insert("username".into(), user.clone().into());
                login_param.insert(
                    "password".into(),
                    BASE64_STANDARD.encode(pwd.clone().into()),
                );

                if let Ok(response) = client.post(url).form(&login_param).send().await {
                    let redirect_location = response
                        .headers()
                        .get("Location")
                        .unwrap()
                        .to_str()
                        .unwrap();
                    if let Ok(response) = client
                        .get(redirect_location)
                        .headers(DEFAULT_HEADERS.clone())
                        .send()
                        .await
                    {
                        cookies
                            .lock()
                            .unwrap()
                            .add_reqwest_cookies(response.cookies(), &ROOT_VPN_URL)
                            .debug_url_cookies(&ROOT_VPN_URL);
                        return Ok(UniversalSSOLogin {
                            response,
                            login_connect_type: LoginConnectType::WEBVPN,
                        });
                    };
                };
            }
        }
        // connect `cczu` and don't need to redirect
        if response.status() == StatusCode::OK {
            let dom = response.text().await.unwrap();
            let mut login_param = parse_hidden_values(dom.as_str());
            login_param.insert("username".into(), user.into());
            login_param.insert("password".into(), BASE64_STANDARD.encode(pwd.into()));

            if let Ok(response) = client.post(ROOT_SSO_LOGIN).form(&login_param).send().await {
                return Ok(UniversalSSOLogin {
                    response,
                    login_connect_type: LoginConnectType::COMMON,
                });
            };
        }
    }
    Err("Can't login! Please check your account!".into())
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
