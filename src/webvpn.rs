use crate::client::UserClient;
use crate::cookies_copy::CopyCookies;
use crate::fields::{DEFAULT_HEADERS, ROOT_SSO, ROOT_VPN};
use crate::types::{
    CbcAES128Enc, ElinkLoginInfo, ElinkServiceInfo, ElinkUserInfo, ElinkUserServiceInfo,
};
use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};
use base64::{prelude::BASE64_STANDARD, Engine};
use rand::Rng;
use reqwest::Url;
use reqwest::{cookie::Cookie, redirect::Policy, Client, StatusCode};
use reqwest_cookie_store::CookieStoreMutex;
use scraper::{Html, Selector};
use std::{collections::HashMap, sync::Arc};
pub struct WebVpnClient {
    pub user: String,
    pub pwd: String,
    pub client: Client,
    pub cookies: Arc<CookieStoreMutex>,
}

fn parse_hidden_values(html: &str) -> HashMap<String, String> {
    let mut hidden_values = HashMap::new();
    let dom = Html::parse_document(html);
    let input_hidden_selector = Selector::parse(r#"input[type="hidden"]"#).unwrap();
    let tags_hidden = dom.select(&input_hidden_selector);

    tags_hidden.for_each(|tag_hidden| {
        hidden_values.insert(
            tag_hidden.attr("name").unwrap().to_string(),
            tag_hidden.attr("value").unwrap().to_string(),
        );
    });

    hidden_values
}

impl WebVpnClient {
    pub fn new(user: &str, pwd: &str) -> Self {
        let cookies = Arc::new(CookieStoreMutex::default());
        WebVpnClient {
            user: user.to_string(),
            pwd: pwd.to_string(),
            client: Client::builder()
                .cookie_provider(cookies.clone())
                .redirect(Policy::none())
                .build()
                .unwrap(),
            cookies: cookies.clone(),
        }
    }

    /// SSO Login
    /// if Ok, return `ElinkUserInfo` else return Err(message)
    pub async fn sso_login(&self) -> Result<ElinkLoginInfo, String> {
        let mut dom = String::new();

        let url = format!(
            "{}/sso/login?service={}/enlink/api/client/callback/cas",
            ROOT_SSO, ROOT_VPN
        );

        if let Ok(response) = self
            .client
            .get(url.clone())
            .headers(DEFAULT_HEADERS.clone())
            .send()
            .await
        {
            let text = response.text().await;

            if let Ok(text) = text {
                dom = text;
            }

            self.cookies.lock().unwrap().copy_cookies(
                &url.parse::<Url>().unwrap(),
                &ROOT_SSO.parse::<Url>().unwrap(),
            )
        }

        if dom.is_empty() {
            // println!("j_session_id: {};dom: {}", j_session_id, dom);
            return Err("SSO 登录失败(无法访问)，请尝试普通登录...".into());
        }

        let mut login_param = parse_hidden_values(dom.as_str());
        login_param.insert("username".into(), self.user.clone());
        login_param.insert("password".into(), BASE64_STANDARD.encode(self.pwd.clone()));

        if let Ok(response) = self
            .client
            .post(format!("{}/sso/login", ROOT_SSO))
            .headers(DEFAULT_HEADERS.clone())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&login_param)
            .send()
            .await
        {
            if response.status() == StatusCode::FOUND {
                let redirect_location = response
                    .headers()
                    .get("Location")
                    .unwrap()
                    .to_str()
                    .unwrap();
                if let Ok(response) = self
                    .client
                    .get(redirect_location)
                    .headers(DEFAULT_HEADERS.clone())
                    .send()
                    .await
                {
                    if let Some(cookie) = &response
                        .cookies()
                        .filter(|cookie| cookie.name() == "clientInfo")
                        .collect::<Vec<Cookie>>()
                        .first()
                    {
                        let json =
                            String::from_utf8(BASE64_STANDARD.decode(cookie.value()).unwrap())
                                .unwrap();
                        //println!("{}", json);
                        return Ok(serde_json::from_str(json.as_str()).unwrap());
                    }
                }
            }
        }

        Err("SSO 登录失败，请尝试普通登录...".into())
    }

    pub async fn common_login(&self) -> Result<ElinkLoginInfo, String> {
        let url = format!("{}/enlink/sso/login/submit", ROOT_VPN);
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut rng = rand::thread_rng();
        let mut token = (0..16)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as u8
            })
            .collect::<Vec<u8>>();
        let iv = token.clone();
        token.reverse();
        let key = token.clone();
        let encryptor = CbcAES128Enc::new(key.as_slice().into(), iv.as_slice().into());
        let pwd_clone = self.pwd.clone();
        let raw_pwd = pwd_clone.as_bytes();
        let pwd_len = raw_pwd.len();
        let mut buf = [0u8; 256];
        buf[..pwd_len].copy_from_slice(&raw_pwd);
        let encrypt_buf = encryptor
            .encrypt_padded_mut::<Pkcs7>(&mut buf, pwd_len)
            .unwrap();
        let encrypt_pwd = BASE64_STANDARD.encode(encrypt_buf);
        let mut data: HashMap<&'static str, String> = HashMap::new();
        data.insert("username", self.user.clone());
        data.insert("password", encrypt_pwd);
        data.insert(
            "token",
            token.iter().map(|char| char.clone() as char).collect(),
        );
        data.insert("language", "zh-CN,zh;q=0.9,en;q=0.8".into());
        // Add Cookies here
        // self.cookies
        //    .add_cookie_str(cookie, &url.parse::<Url>().unwrap());
        if let Ok(response) = self
            .client
            .post(url)
            .header("Refer", format!("{}/enlink/sso/login", ROOT_VPN))
            .header("Origin", ROOT_VPN)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .headers(DEFAULT_HEADERS.clone())
            .form(&data)
            .send()
            .await
        {
            if response.status() == StatusCode::FOUND {
                if let Some(cookie) = &response
                    .cookies()
                    .filter(|cookie| cookie.name() == "clientInfo")
                    .collect::<Vec<Cookie>>()
                    .first()
                {
                    let json =
                        String::from_utf8(BASE64_STANDARD.decode(cookie.value()).unwrap()).unwrap();
                    return Ok(serde_json::from_str(json.as_str()).unwrap());
                }
            }
        };
        Err("普通登录失败，请检查账号密码是否错误...".into())
    }

    pub async fn get_user_info(&self, user_id: String) -> Result<ElinkUserInfo, String> {
        if let Ok(response) = self
            .client
            .get(format!(
                "{}/enlink/api/client/user/findByUserId/{}",
                ROOT_VPN, user_id
            ))
            .headers(DEFAULT_HEADERS.clone())
            .send()
            .await
        {
            if let Ok(json) = response.text().await {
                return Ok(serde_json::from_str(json.as_str()).unwrap());
            }
        }
        Err("获取失败，请稍后重试".into())
    }

    pub async fn get_tree_with_service(&self, user_id: String) -> Result<ElinkServiceInfo, String> {
        let mut body = HashMap::new();
        body.insert("nameLike", "");
        body.insert("serviceNameLike", "");
        body.insert("userId", user_id.as_str());
        if let Ok(response) = self
            .client
            .post(format!(
                "{}/enlink/api/client/service/group/treeWithService/",
                ROOT_VPN
            ))
            .headers(DEFAULT_HEADERS.clone())
            .header("Referer", format!("{}/enlink/", ROOT_VPN))
            .header("Origin", ROOT_VPN)
            .header("Content-Type", "application/json;charset=utf-8")
            .body(serde_json::to_string(&body).unwrap())
            .send()
            .await
        {
            if let Ok(json) = response.text().await {
                return Ok(serde_json::from_str(json.as_str()).unwrap());
            }
        }
        Err("获取失败，请稍后重试".into())
    }
    pub async fn get_service_by_user(
        &self,
        user_id: String,
    ) -> Result<ElinkUserServiceInfo, String> {
        let mut param = HashMap::new();
        param.insert("name", "");
        if let Ok(response) = self
            .client
            .get(format!(
                "{}/enlink/api/client/service/sucmp/findServiceByUserId/{}",
                ROOT_VPN, user_id
            ))
            .headers(DEFAULT_HEADERS.clone())
            .header("Referer", format!("{}/enlink/", ROOT_VPN))
            .header("Origin", ROOT_VPN)
            .query(&param)
            .send()
            .await
        {
            if let Ok(json) = response.text().await {
                return Ok(serde_json::from_str(json.as_str()).unwrap());
            }
        }
        Err("获取失败，请稍后重试".into())
    }
}

impl UserClient for WebVpnClient {
    fn login(&self) {
        todo!()
    }

    fn get_client(&self) -> &Client {
        &self.client
    }

    fn get_client_mut(&mut self) -> &mut Client {
        &mut self.client
    }
}
