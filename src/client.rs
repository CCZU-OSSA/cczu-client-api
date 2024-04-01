use std::sync::Arc;

use crate::app::base::Application;
use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;

pub trait UserClient {
    fn get_client(&self) -> Arc<Client>;
    fn get_client_mut(&mut self) -> Arc<Client>;
    fn get_cookies(&self) -> Arc<CookieStoreMutex>;
    fn get_cookies_mut(&mut self) -> Arc<CookieStoreMutex>;
    /// redirect the url if use webvpn.
    fn redirect(&self, url: &str) -> String;
    /// copy the login cookie to the url.
    fn initialize_url(&self, url: &str);
    fn get_user(&self) -> String;
    fn get_pwd(&self) -> String;
}

impl dyn UserClient {
    pub fn visit_application<'a, T>(&'a mut self) -> T
    where
        T: Application<'a>,
    {
        T::from_client(self)
    }
}
