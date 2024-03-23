use std::sync::Arc;

use crate::app::app::Application;
use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;

pub trait UserClient {
    fn login(&self);
    fn get_client(&self) -> Arc<Client>;
    fn get_client_mut(&mut self) -> Arc<Client>;
    fn get_cookies(&self) -> Arc<CookieStoreMutex>;
    fn get_cookies_mut(&mut self) -> Arc<CookieStoreMutex>;
    fn host(&self, url: &str) -> String;
}

impl dyn UserClient {
    pub fn visit_application<'a, T>(&'a mut self) -> T
    where
        T: Application<'a>,
    {
        T::from_client(self)
    }
}
