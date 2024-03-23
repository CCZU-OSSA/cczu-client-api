use crate::client::UserClient;

pub struct CommonClient {}

impl UserClient for CommonClient {
    fn login(&self) {
        todo!()
    }

    fn get_cookies(&self) -> std::sync::Arc<reqwest_cookie_store::CookieStoreMutex> {
        todo!()
    }

    fn get_cookies_mut(&mut self) -> std::sync::Arc<reqwest_cookie_store::CookieStoreMutex> {
        todo!()
    }

    fn host(&self, _url: &str) -> String {
        todo!()
    }

    fn get_client(&self) -> std::sync::Arc<reqwest::Client> {
        todo!()
    }

    fn get_client_mut(&mut self) -> std::sync::Arc<reqwest::Client> {
        todo!()
    }
}
