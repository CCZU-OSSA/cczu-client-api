use crate::client::UserClient;

use super::app::Application;
pub struct JwcasApplication<'a> {
    client: &'a mut dyn UserClient,
}

impl<'a> Application<'a> for JwcasApplication<'a> {
    fn from_client(client: &'a mut dyn UserClient) -> Self {
        JwcasApplication::new(client)
    }
}

impl<'a> JwcasApplication<'a> {
    pub fn new(client: &'a mut dyn UserClient) -> Self {
        Self { client }
    }

    pub fn get_classes(&mut self) {
        let client_cookies = self.client.get_cookies_mut();
        todo!()
    }
}
