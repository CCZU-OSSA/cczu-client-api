use std::sync::Arc;

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
