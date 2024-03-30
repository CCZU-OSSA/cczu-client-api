use std::sync::{Arc, Mutex};

use crate::{
    client::UserClient,
    common::CommonClient,
    sso::{is_webvpn_available, universal_sso_login},
    types::LoginConnectType,
    webvpn::WebVpnClient,
};
use reqwest::{redirect::Policy, Client, ClientBuilder};
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
        Self::new(CommonClient::new(user, pwd))
    }

    pub fn webvpn(user: String, pwd: String) -> Self {
        Self::new(WebVpnClient::new(user, pwd))
    }

    pub fn webvpn_custom(
        client: Arc<Client>,
        cookies: Arc<CookieStoreMutex>,
        user: String,
        pwd: String,
    ) -> Self {
        Self::new(WebVpnClient::from_custom(client, cookies, user, pwd))
    }

    pub fn common_custom(
        client: Arc<Client>,
        cookies: Arc<CookieStoreMutex>,
        user: String,
        pwd: String,
    ) -> Self {
        Self::new(CommonClient::from_custom(client, cookies, user, pwd))
    }

    pub async fn auto(user: String, pwd: String) -> Self {
        if is_webvpn_available().await {
            Self::webvpn(user, pwd)
        } else {
            Self::common(user, pwd)
        }
    }

    pub async fn auto_login(user: String, pwd: String) -> Self {
        let cookies = Arc::new(CookieStoreMutex::default());
        let client = Arc::new(
            ClientBuilder::new()
                .redirect(Policy::none())
                .cookie_provider(cookies.clone())
                .build()
                .unwrap(),
        );
        let login_info =
            universal_sso_login(client.clone(), cookies.clone(), user.clone(), pwd.clone())
                .await
                .unwrap();

        match login_info.login_connect_type {
            LoginConnectType::COMMON => Self::common_custom(client, cookies, user, pwd),
            LoginConnectType::WEBVPN => Self::webvpn_custom(client, cookies, user, pwd),
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
        self.client.lock().unwrap().initialize_url(url);
    }

    fn get_user(&self) -> String {
        self.client.lock().unwrap().get_user()
    }

    fn get_pwd(&self) -> String {
        self.client.lock().unwrap().get_pwd()
    }
}
