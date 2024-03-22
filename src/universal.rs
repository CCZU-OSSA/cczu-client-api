use crate::{client::UserClient, webvpn::WebVpnClient};
use reqwest_cookie_store::CookieStoreMutex;
pub enum ClientType {
    WebVPN,
    Common,
}
/// Universal Can chose the ClientType.
pub struct UniversalClient {
    client: Box<dyn UserClient>,
}

impl UniversalClient {
    pub fn new(client: ClientType, user: &str, pwd: &str) -> Self {
        Self {
            client: Box::new(match client {
                ClientType::WebVPN => WebVpnClient::new(user, pwd),
                _ => todo!(),
            }),
        }
    }

    pub fn entries() -> i32 {
        1
    }
}

impl UserClient for UniversalClient {
    fn login(&self) {
        self.client.login()
    }

    fn get_client(&self) -> &reqwest::Client {
        self.client.get_client()
    }

    fn get_client_mut(&mut self) -> &mut reqwest::Client {
        self.client.get_client_mut()
    }

    fn get_cookies(&self) -> std::sync::Arc<CookieStoreMutex> {
        self.client.get_cookies()
    }

    fn get_cookies_mut(&mut self) -> std::sync::Arc<CookieStoreMutex> {
        self.client.get_cookies_mut()
    }

    fn host(&self, _url: &str) -> String {
        todo!()
    }
}
