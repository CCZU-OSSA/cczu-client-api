use std::collections::HashMap;

use once_cell::sync::Lazy;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Url,
};

pub static DEFAULT_HEADERS: Lazy<HeaderMap> = Lazy::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert(
        "User-Agent",
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/116.0",
        ),
    );
    headers
});
pub const ROOT_SSO: &'static str = "http://sso.cczu.edu.cn";
pub const ROOT_SSO_URL: Lazy<Url> = Lazy::new(|| Url::parse(ROOT_SSO).unwrap());
pub const ROOT_SSO_LOGIN: &'static str = "http://sso.cczu.edu.cn/sso/login";
// pub const ROOT_SSO_LOGIN_URL: Lazy<Url> = Lazy::new(|| Url::parse(ROOT_SSO_LOGIN).unwrap());

pub const ROOT_VPN: &'static str = "https://zmvpn.cczu.edu.cn";
pub const ROOT_VPN_URL: Lazy<Url> = Lazy::new(|| Url::parse(ROOT_VPN).unwrap());

pub const ROOT_YWTB: &'static str = "http://ywtb.cczu.edu.cn";
pub const WEBVPN_SERVER_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // jwcas
    map.insert(
        "http://219.230.159.132".into(),
        "https://zmvpn.cczu.edu.cn/http/webvpndc2d086cb5b297c15e661687e73c1549".into(),
    );

    map
});

pub const WECHAT_APP_API: &'static str = "http://202.195.102.7:8180";
