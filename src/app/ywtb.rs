use crate::client::UserClient;

use super::app::Application;

pub struct YwtbApplication<'a> {
    pub client: &'a mut dyn UserClient,
    pub root: String,
}

impl<'a> Application<'a> for YwtbApplication<'a> {
    fn from_client(client: &'a mut dyn UserClient) -> Self {
        let root = client.redirect("http://ywtb.cczu.edu.cn");
        Self { client, root }
    }
}
