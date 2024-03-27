use async_recursion::async_recursion;
use reqwest::{redirect::Policy, Client, Method, Response, StatusCode};
use std::{collections::HashMap, sync::Arc};

use crate::{fields::DEFAULT_HEADERS, types::Resp};

pub struct Request {
    pub url: String,
    pub method: Method,
    pub cookies: HashMap<String, String>,
    pub cookie: bool,
    redirect: bool,
    param: HashMap<String, String>,
    headers: HashMap<String, String>,
    body: String,
    form: HashMap<String, String>,
    client: Arc<Client>,
}

impl Request {
    pub fn new(url: String, method: Method) -> Self {
        return Request {
            url: url.clone(),
            method: method.clone(),
            param: HashMap::new(),
            headers: HashMap::new(),
            body: "".into(),
            form: HashMap::new(),
            cookies: HashMap::new(),
            cookie: true,
            redirect: false,
            client: Arc::new(Client::builder().redirect(Policy::none()).build().unwrap()),
        };
    }

    pub fn get(url: String) -> Self {
        return Request::new(url, Method::GET);
    }

    pub fn post(url: String) -> Self {
        return Request::new(url, Method::POST);
    }

    pub fn cookie(&mut self, cookie: bool) -> &mut Self {
        self.cookie = cookie;
        self
    }

    /**
     * 是否跳转跟随方法
     * 如果否，则跳转一律使用get带cookie请求
     */
    pub fn follow_redirect(&mut self, redirect: bool) -> &mut Self {
        self.redirect = redirect;
        self
    }

    pub fn param(&mut self, key: &str, value: &str) -> &mut Self {
        self.param.insert(key.into(), value.into());
        self
    }
    pub fn header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn form(&mut self, key: &str, value: &str) -> &mut Self {
        self.form.insert(key.into(), value.into());
        self
    }

    pub fn forms(&mut self, map: HashMap<String, String>) -> &mut Self {
        self.form = map.clone();
        self
    }

    pub fn cookies(&mut self, map: HashMap<String, String>) -> &mut Self {
        self.cookies = map.clone();
        self
    }

    pub fn body(&mut self, body: &str) -> &mut Self {
        self.body = body.into();
        self
    }

    pub async fn send(&mut self) -> Result<Resp, String> {
        let mut executor = self.client.request(self.method.clone(), self.url.clone());
        // 判断 param
        if !self.param.is_empty() {
            executor = executor.query(&self.param.clone());
        }
        // 判断 form
        if !self.form.is_empty() {
            executor = executor
                .form(&self.form.clone())
                .header("Content-Type", "application/x-www-form-urlencoded");
        }
        // 判断 body
        if !self.body.is_empty() {
            executor = executor.body(self.body.clone());
        }
        // 携带头
        if !self.headers.is_empty() {
            // 添加默认头
            executor = executor.headers(DEFAULT_HEADERS.clone());
            for (key, value) in self.headers.clone() {
                executor = executor.header(key, value)
            }
        }
        // 拼接 Cookies
        executor = executor.header(
            "Cookie",
            self.cookies
                .iter()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect::<Vec<String>>()
                .join(";"),
        );

        if let Ok(resp) = executor.send().await {
            return self.parse_respose(resp).await;
        }
        Err("Failed.".into())
    }

    #[async_recursion]
    async fn parse_respose(&mut self, resp: Response) -> Result<Resp, String> {
        // 处理响应
        // 存储cookie

        resp.headers()
            .iter()
            .filter(|(key, _)| key.to_string().to_ascii_lowercase() == "set-cookie")
            .for_each(|(_, value)| {
                let cookie: String = urlencoding::decode(value.to_str().unwrap()).unwrap().into();
                let mut cookies = cookie.split(";");
                let c: String = cookies.next().unwrap().into();
                let mut sc = c.split("=").clone();
                let cookie_key: String = sc.next().unwrap().into();
                let cookie_value: String = sc.next().unwrap().into();

                self.cookies
                    .insert(cookie_key.clone(), cookie_value.clone());
                println!("Add cookies: {}={}", cookie_key, cookie_value);
            });

        if resp.status() == StatusCode::FOUND {
            // 取Location头
            let location = resp.headers().get("Location").unwrap().to_str().unwrap();
            println!("redirect to: {}", location);
            if self.redirect {
                self.url = location.into();
                return self.send().await;
            }
            return Request::get(location.into())
                .cookies(self.cookies.clone())
                .send()
                .await;
        }
        Ok(Resp {
            resp,
            cookies: self.cookies.clone(),
        })
    }
}
