use std::sync::{Arc, Mutex};

use crate::{
    app::base::Application,
    client::UserClient,
    common::CommonClient,
    sso::{is_webvpn_available, session_available, universal_sso_login},
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

    pub fn common(user: String, password: String) -> Self {
        Self::new(CommonClient::new(user, password))
    }

    pub fn webvpn(user: String, password: String) -> Self {
        Self::new(WebVpnClient::new(user, password))
    }

    pub fn webvpn_custom(
        client: Arc<Client>,
        cookies: Arc<CookieStoreMutex>,
        user: String,
        password: String,
    ) -> Self {
        Self::new(WebVpnClient::from_custom(client, cookies, user, password))
    }

    pub fn common_custom(
        client: Arc<Client>,
        cookies: Arc<CookieStoreMutex>,
        user: String,
        password: String,
    ) -> Self {
        Self::new(CommonClient::from_custom(client, cookies, user, password))
    }

    pub async fn auto(user: String, password: String) -> Self {
        if is_webvpn_available().await {
            Self::webvpn(user, password)
        } else {
            Self::common(user, password)
        }
    }

    pub async fn from_cookies(user: String, password: String, cookies: String) -> Option<Self> {
        let provider: reqwest_cookie_store::CookieStore = serde_json::from_str(&cookies).unwrap();
        let cookies = Arc::new(CookieStoreMutex::new(provider));
        let client = Arc::new(
            ClientBuilder::new()
                .redirect(Policy::none())
                .cookie_provider(cookies.clone())
                .build()
                .unwrap(),
        );
        if session_available(client.clone()).await {
            if is_webvpn_available().await {
                Some(Self::webvpn_custom(client, cookies, user, password))
            } else {
                Some(Self::common_custom(client, cookies, user, password))
            }
        } else {
            if let Ok(client) = Self::auto_login(user, password).await {
                Some(client)
            } else {
                None
            }
        }
    }

    pub async fn auto_login(user: String, password: String) -> Result<Self, String> {
        let cookies = Arc::new(CookieStoreMutex::default());
        let client = Arc::new(
            ClientBuilder::new()
                .redirect(Policy::none())
                .cookie_provider(cookies.clone())
                .build()
                .unwrap(),
        );

        let login_result = universal_sso_login(
            client.clone(),
            cookies.clone(),
            user.clone(),
            password.clone(),
        )
        .await;
        if let Err(message) = login_result {
            return Err(message);
        } else {
            if !session_available(client.clone()).await {
                return Err("登录失败, 账户密码错误?".into());
            }

            return match login_result.unwrap().login_connect_type {
                LoginConnectType::COMMON => {
                    Ok(Self::common_custom(client, cookies, user, password))
                }
                LoginConnectType::WEBVPN => {
                    Ok(Self::webvpn_custom(client, cookies, user, password))
                }
            };
        };
    }

    pub fn visitor(&self) -> Arc<Mutex<dyn UserClient>> {
        self.client.clone()
    }

    pub fn visit_application<'a, T>(&'a mut self) -> T
    where
        T: Application<'a>,
    {
        T::from_client(self)
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
