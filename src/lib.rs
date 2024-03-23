pub mod app;
pub mod client;
pub mod common;
pub mod cookies_io;
pub mod fields;
pub mod types;
pub mod universal;
pub mod webvpn;
#[cfg(test)]
mod test {
    use super::webvpn::WebVpnClient;
    use super::{
        app::jwcas::JwcasApplication,
        universal::{ClientType, UniversalClient},
    };
    use rand::Rng;

    #[tokio::test]

    async fn login_test() {
        let mut client = WebVpnClient::new("账号", "密码");

        match client.common_login().await {
            Ok(json) => {
                println!("{:?}", json);
                // The UserInfo test
                match client.get_user_info().await {
                    Ok(json) => println!("{:?}", json),
                    Err(message) => println!("{}", message),
                }

                // The user service test
                match client.get_service_by_user().await {
                    Ok(json) => println!("{:?}", json),
                    Err(message) => println!("{}", message),
                }

                // The tree service test
                match client.get_tree_with_service().await {
                    Ok(json) => println!("{:?}", json),
                    Err(message) => println!("{}", message),
                }
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
        let universal = UniversalClient::new(ClientType::WebVPN, "账号", "密码");
        let _app = universal
            .visitor()
            .lock()
            .unwrap()
            .visit_application::<JwcasApplication>();
    }
}
