pub mod fields;
pub mod webvpn;

#[cfg(test)]
mod test {
    use super::webvpn::WebVpnClient;

    #[tokio::test]
    async fn login_test() {
        let client = WebVpnClient::new("学号", "密码");
        match client.sso_login().await {
            Ok(json) => println!("{}", json),
            Err(message) => println!("{}", message),
        };
    }
}
