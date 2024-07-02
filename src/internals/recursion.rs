use std::sync::Arc;

use async_recursion::async_recursion;
use reqwest::{Client, Response, StatusCode, Url};
use reqwest_cookie_store::CookieStoreMutex;

use crate::internals::{cookies_io::CookiesIOExt, fields::DEFAULT_HEADERS};

#[async_recursion]
pub async fn recursion_cookies_handle(
    client: Arc<Client>,
    cookies: Arc<CookieStoreMutex>,
    url: &str,
    cookie_store_url: &Url,
) -> Result<Response, String> {
    if let Ok(response) = client
        .get(url)
        .headers(DEFAULT_HEADERS.clone())
        .send()
        .await
    {
        cookies
            .lock()
            .unwrap()
            .copy_cookies_raw(&Url::parse(url).unwrap(), cookie_store_url);
        if response.status() == StatusCode::FOUND {
            return recursion_cookies_handle(
                client,
                cookies,
                response
                    .headers()
                    .get("location")
                    .unwrap()
                    .to_str()
                    .unwrap(),
                cookie_store_url,
            )
            .await;
        }
        return Ok(response);
    }

    Err(format!("Can't get `{}`", url))
}
