use std::sync::Arc;

use crate::app::app::Application;
use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;

pub trait UserClient {
    fn login(&self);
    fn get_client(&self) -> &Client;
    fn get_client_mut(&mut self) -> &mut Client;
    fn get_cookies(&self) -> Arc<CookieStoreMutex>;
    fn get_cookies_mut(&mut self) -> Arc<CookieStoreMutex>;
    fn host(&self, url: &str) -> String;
}

impl dyn UserClient {
    pub fn visit_application<T>(&mut self) -> T
    where
        T: Application,
    {
        T::from_client(self)
    }
}
