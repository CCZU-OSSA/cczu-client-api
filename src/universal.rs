use std::sync::{Arc, Mutex};

use crate::{client::UserClient, webvpn::WebVpnClient};
use reqwest_cookie_store::CookieStoreMutex;
pub enum ClientType {
    WebVPN,
    Common,
}
/// Universal Can chose the ClientType.
pub struct UniversalClient {
    client: Arc<Mutex<dyn UserClient>>,
}

impl UniversalClient {
    pub fn new(client: ClientType, user: &str, pwd: &str) -> Self {
        Self {
            client: Arc::new(Mutex::new(match client {
                ClientType::WebVPN => WebVpnClient::new(user, pwd),
                _ => todo!(),
            })),
        }
    }
    pub fn visitor(&self) -> Arc<Mutex<dyn UserClient>> {
        self.client.clone()
    }
}

impl UserClient for UniversalClient {
    fn login(&self) {
        self.client.lock().unwrap().login()
    }

    fn get_cookies(&self) -> Arc<CookieStoreMutex> {
        self.client.lock().unwrap().get_cookies()
    }

    fn get_cookies_mut(&mut self) -> Arc<CookieStoreMutex> {
        self.client.lock().unwrap().get_cookies_mut()
    }

    fn host(&self, _url: &str) -> String {
        todo!()
    }

    fn get_client(&self) -> Arc<reqwest::Client> {
        self.client.lock().unwrap().get_client()
    }

    fn get_client_mut(&mut self) -> Arc<reqwest::Client> {
        self.client.lock().unwrap().get_client_mut()
    }
}
