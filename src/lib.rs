pub mod client;
pub mod fields;
pub mod cookies_copy;
pub mod types;
pub mod webvpn;
#[cfg(test)]
mod test {
    use super::webvpn::WebVpnClient;
    use rand::Rng;

    #[tokio::test]

    async fn login_test() {
        let client = WebVpnClient::new("账号", "密码");
        match client.common_login().await {
            Ok(json) => {
                println!("{:?}", json);
                match client.get_user_info(json.userid.clone().unwrap()).await {
                    Ok(json) => println!("{:?}", json),
                    Err(message) => println!("{}", message),
                }
                match client
                    .get_service_by_user(json.userid.clone().unwrap())
                    .await
                {
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
}
