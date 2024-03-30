pub mod app;
pub mod client;
pub mod common;
pub mod cookies_io;
pub mod fields;
pub mod recursion;
pub mod sso;
pub mod types;
pub mod universal;
pub mod webvpn;
#[cfg(test)]
mod test {
    use crate::app::app::Application;

    use super::common::CommonClient;

    use super::webvpn::WebVpnClient;
    use super::{app::jwcas::JwcasApplication, universal::UniversalClient};
    use rand::Rng;

    const USER: &'static str = "";
    const PWD: &'static str = "";

    #[tokio::test]

    async fn login_test() {
        let mut client = WebVpnClient::new(USER.into(), PWD.into());

        match client.sso_login().await {
            Ok(json) => {
                println!("{:?}", json);
            }
            Err(message) => println!("{}", message),
        };
    }
    #[test]
    fn random_string_test() {
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut rng = rand::thread_rng();
        let token: String = (0..16)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        println!("{}", token)
    }

    #[tokio::test]
    async fn universal_test() {
        let _ = UniversalClient::auto_login(USER.into(), PWD.into()).await;
    }

    #[tokio::test]
    async fn common_test() {
        let mut client = CommonClient::new(USER.into(), PWD.into());
        client.sso_login().await.unwrap();
        let app = JwcasApplication::from_client(&mut client);
        app.get_class_list().await;
    }

    #[tokio::test]
    async fn webvpn_test() {
        let mut client = WebVpnClient::new(USER.into(), PWD.into());
        client.sso_login().await.unwrap();
        let app = JwcasApplication::from_client(&mut client);
        app.get_class_list().await;
    }
}
