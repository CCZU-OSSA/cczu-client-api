pub mod client;
pub mod fields;
pub mod types;

#[cfg(test)]
mod test {
    use rand::Rng;
    use super::client::WebVpnClient;

    #[tokio::test]
    async fn login_test() {
        let client = WebVpnClient::new("学号", "密码");
        match client.sso_login().await {
            Ok(json) => println!("{:?}", json),
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
