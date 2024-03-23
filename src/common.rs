use std::sync::Arc;

use reqwest::{Client, ClientBuilder, Url};
use reqwest_cookie_store::CookieStoreMutex;

use crate::{
    client::UserClient,
    cookies_io::CookiesIOExt,
    fields::{ROOT_SSO, ROOT_YWTB},
    sso::sso_login,
};

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

    pub async fn sso_login(&self) -> Result<(), String> {
        let result = sso_login(
            self.get_client(),
            self.get_cookies(),
            self.user.clone(),
            self.pwd.clone(),
            ROOT_YWTB.to_owned(),
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

        return Err(result.err().unwrap());
    }
}

impl UserClient for CommonClient {
    fn get_cookies(&self) -> std::sync::Arc<reqwest_cookie_store::CookieStoreMutex> {
        self.cookies.clone()
    }

    fn get_cookies_mut(&mut self) -> std::sync::Arc<reqwest_cookie_store::CookieStoreMutex> {
        self.cookies.clone()
    }

    fn host(&self, _url: &str) -> String {
        todo!()
    }

    fn get_client(&self) -> std::sync::Arc<reqwest::Client> {
        self.client.clone()
    }

    fn get_client_mut(&mut self) -> std::sync::Arc<reqwest::Client> {
        self.client.clone()
    }
}
