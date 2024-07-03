use std::sync::Arc;

use reqwest::{Client, ClientBuilder};
use reqwest_cookie_store::CookieStoreMutex;

use crate::impl_auth_client;

use super::app::Application;

pub trait AuthClient {
    fn get_client(&self) -> Arc<Client>;
    fn get_client_mut(&mut self) -> Arc<Client>;
    fn get_cookies(&self) -> Arc<CookieStoreMutex>;
    fn get_cookies_mut(&mut self) -> Arc<CookieStoreMutex>;

    fn get_user(&self) -> String;
    fn get_pwd(&self) -> String;
}

pub trait Redirect {
    /// redirect the url if use webvpn.
    fn redirect(&self, url: &str) -> String;
    /// copy the login cookie to the url.
    fn initialize_url(&self, url: &str);
}
pub trait SSOClient: AuthClient + Redirect {}
impl<T: AuthClient + Redirect> SSOClient for T {}
#[derive(Clone)]
pub struct SimpleClient {
    pub user: String,
    pub pwd: String,
    pub client: Arc<Client>,
    pub cookies: Arc<CookieStoreMutex>,
}

impl_auth_client!(SimpleClient);
impl SimpleClient {
    pub fn new<S: Into<String>>(user: S, pwd: S) -> Self {
        let cookies = Arc::new(CookieStoreMutex::default());
        Self {
            user: user.into(),
            pwd: pwd.into(),
            client: Arc::new(
                ClientBuilder::new()
                    .cookie_provider(cookies.clone())
                    .build()
                    .unwrap(),
            ),
            cookies: cookies.clone(),
        }
    }

    pub fn from_custom<S: Into<String>>(
        client: Arc<Client>,
        cookies: Arc<CookieStoreMutex>,
        user: S,
        pwd: S,
    ) -> Self {
        Self {
            user: user.into(),
            pwd: pwd.into(),
            client,
            cookies,
        }
    }
}

pub trait Visitor {
    fn visit_application<T: Application>(&self) -> T;
}

impl Visitor for SimpleClient {
    fn visit_application<T: Application>(&self) -> T {
        T::from_client(Arc::new(self.clone()))
    }
}
