use reqwest::Url;

use super::session::universal_sso_login;
use crate::base::client::{AuthClient, Redirect, SimpleClient};
use crate::internals::cookies_io::CookiesIOExt;
use crate::internals::fields::{ROOT_SSO, ROOT_SSO_URL, ROOT_YWTB};

pub trait SSOSimpleExt {
    fn sso_login(&self) -> impl std::future::Future<Output = Result<(), String>> + Send;
}

impl SSOSimpleExt for SimpleClient {
    async fn sso_login(&self) -> Result<(), String> {
        let result = universal_sso_login(
            self.get_client(),
            self.get_cookies(),
            self.user.clone(),
            self.pwd.clone(),
        )
        .await;
        if let Ok(_) = result {
            self.cookies.lock().unwrap().copy_cookies(
                &ROOT_SSO.parse::<Url>().unwrap(),
                &format!("{}/pc/index.html", ROOT_YWTB)
                    .parse::<Url>()
                    .unwrap(),
            );

            return Ok(());
        };

        Err(result.err().unwrap())
    }
}

impl Redirect for SimpleClient {
    fn initialize_url(&self, url: &str) {
        self.get_cookies()
            .lock()
            .unwrap()
            .copy_cookies_raw(&ROOT_SSO_URL, &Url::parse(url).unwrap());
    }

    fn redirect(&self, url: &str) -> String {
        url.to_string()
    }
}
