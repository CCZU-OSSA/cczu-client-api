use std::sync::{Arc, Mutex};

use crate::{
    client::UserClient, common::CommonClient, cookies_io::CookiesIOExt, fields::ROOT_SSO_URL,
    sso::is_webvpn_available, webvpn::WebVpnClient,
};
use reqwest::Url;
use reqwest_cookie_store::CookieStoreMutex;

/// Universal Can chose the ClientType.
pub struct UniversalClient {
    client: Arc<Mutex<dyn UserClient>>,
}

impl UniversalClient {
    pub fn new(client: impl UserClient + 'static) -> Self {
        Self {
            client: Arc::new(Mutex::new(client)),
        }
    }

    pub fn common(user: String, pwd: String) -> Self {
        Self {
            client: Arc::new(Mutex::new(CommonClient::new(user, pwd))),
        }
    }

    pub fn webvpn(user: String, pwd: String) -> Self {
        Self {
            client: Arc::new(Mutex::new(WebVpnClient::new(user, pwd))),
        }
    }

    pub async fn auto(user: String, pwd: String) -> Self {
        if is_webvpn_available().await {
            Self::webvpn(user, pwd)
        } else {
            Self::common(user, pwd)
        }
    }

    pub fn visitor(&self) -> Arc<Mutex<dyn UserClient>> {
        self.client.clone()
    }
}

impl UserClient for UniversalClient {
    fn get_cookies(&self) -> Arc<CookieStoreMutex> {
        self.client.lock().unwrap().get_cookies()
    }

    fn get_cookies_mut(&mut self) -> Arc<CookieStoreMutex> {
        self.client.lock().unwrap().get_cookies_mut()
    }

    fn redirect(&self, url: &str) -> String {
        self.client.lock().unwrap().redirect(url)
    }

    fn get_client(&self) -> Arc<reqwest::Client> {
        self.client.lock().unwrap().get_client()
    }

    fn get_client_mut(&mut self) -> Arc<reqwest::Client> {
        self.client.lock().unwrap().get_client_mut()
    }

    fn initialize_url(&self, url: &str) {
        self.get_cookies()
            .lock()
            .unwrap()
            .copy_cookies(&ROOT_SSO_URL, &Url::parse(url).unwrap());
    }

    fn get_user(&self) -> String {
        self.client.lock().unwrap().get_user()
    }

    fn get_pwd(&self) -> String {
        self.client.lock().unwrap().get_pwd()
    }
}
