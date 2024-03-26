use reqwest::Url;

use crate::{client::UserClient, cookies_io::CookiesIOExt, fields::ROOT_VPN, sso::sso_login};

use super::app::Application;

pub struct YwtbApplication<'a> {
    client: &'a mut dyn UserClient,
    root: String,
}

impl<'a> Application<'a> for YwtbApplication<'a> {
    fn from_client(client: &'a mut dyn UserClient) -> Self {
        let root = client.redirect("http://ywtb.cczu.edu.cn");
        Self { client, root }
    }
}

impl<'a> YwtbApplication<'a> {
    pub async fn login(&self) {
        if let Ok(_) = sso_login(
            self.client.get_client(),
            self.client.get_cookies(),
            self.client.get_user(),
            self.client.get_pwd(),
            "http://ywtb.cczu.edu.cn/pc/index.html".to_owned(),
        )
        .await
        {
            self.client.get_cookies().lock().unwrap().copy_cookies(
                &Url::parse(&self.root).unwrap(),
                &Url::parse(ROOT_VPN).unwrap(),
            );
        };
    }
}
