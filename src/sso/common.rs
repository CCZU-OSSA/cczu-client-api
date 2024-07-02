use std::sync::Arc;

use reqwest::{Client, ClientBuilder, Url};
use reqwest_cookie_store::CookieStoreMutex;

use super::session::universal_sso_login;
use crate::base::client::UserClient;
use crate::internal::cookies_io::CookiesIOExt;
use crate::internal::fields::{ROOT_SSO, ROOT_SSO_URL, ROOT_YWTB};
pub struct CommonClient {
    pub user: String,
    pub pwd: String,
    pub client: Arc<Client>,
    pub cookies: Arc<CookieStoreMutex>,
}

impl CommonClient {
    pub fn new(user: String, pwd: String) -> Self {
        let cookies = Arc::new(CookieStoreMutex::default());
        Self {
            user,
            pwd,
            client: Arc::new(
                ClientBuilder::new()
                    .cookie_provider(cookies.clone())
                    .build()
                    .unwrap(),
            ),
            cookies: cookies.clone(),
        }
    }

    pub fn from_custom(
        client: Arc<Client>,
        cookies: Arc<CookieStoreMutex>,
        user: String,
        pwd: String,
    ) -> Self {
        Self {
            user,
            pwd,
            client,
            cookies,
        }
    }

    // Not Ready to use.
    pub async fn sso_login(&self) -> Result<(), String> {
        let result = universal_sso_login(
            self.get_client(),
            self.get_cookies(),
            self.user.clone(),
            self.pwd.clone(),
        )
        .await;
        if let Ok(_) = result {
            self.cookies.lock().unwrap().copy_cookies(
                &ROOT_SSO.parse::<Url>().unwrap(),
                &format!("{}/pc/index.html", ROOT_YWTB)
                    .parse::<Url>()
                    .unwrap(),
            );

            return Ok(());
        };

        Err(result.err().unwrap())
    }
}

impl UserClient for CommonClient {
    fn get_cookies(&self) -> Arc<reqwest_cookie_store::CookieStoreMutex> {
        self.cookies.clone()
    }

    fn get_cookies_mut(&mut self) -> Arc<reqwest_cookie_store::CookieStoreMutex> {
        self.cookies.clone()
    }

    fn redirect(&self, url: &str) -> String {
        url.to_string()
    }

    fn get_client(&self) -> std::sync::Arc<reqwest::Client> {
        self.client.clone()
    }

    fn get_client_mut(&mut self) -> std::sync::Arc<reqwest::Client> {
        self.client.clone()
    }

    fn initialize_url(&self, url: &str) {
        self.get_cookies()
            .lock()
            .unwrap()
            .copy_cookies_raw(&ROOT_SSO_URL, &Url::parse(url).unwrap());
    }

    fn get_user(&self) -> String {
        self.user.clone()
    }

    fn get_pwd(&self) -> String {
        self.pwd.clone()
    }
}
