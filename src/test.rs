use tokio::runtime::Builder;

use crate::webvpn::WebVpnClient;

#[test]
fn main() {
    println!("Try Login");
    let webvpn = WebVpnClient::new("学号", "密码");
    Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(webvpn.sso_login());
}
