use reqwest::{cookie::Cookie as ReqwestCookie, Url};
use reqwest_cookie_store::{CookieStore, RawCookie};

pub trait CookiesIOExt {
    fn copy_cookies(&mut self, from: &Url, to: &Url) -> &mut Self;
    fn add_reqwest_cookie(&mut self, cookie: ReqwestCookie, reqwest_url: &Url) -> &mut Self;
    fn add_reqwest_cookies<'a>(
        &mut self,
        cookies: impl Iterator<Item = ReqwestCookie<'a>>,
        reqwest_url: &Url,
    ) -> &mut Self;
}

impl CookiesIOExt for CookieStore {
    /// Copy the cookies of `from` to `to`
    fn copy_cookies(&mut self, from: &Url, to: &Url) -> &mut Self {
        self.clone().matches(from).into_iter().for_each(|cookie| {
            self.insert(cookie.clone(), to).unwrap();
        });
        self
    }

    /// May lost the metadata of cookie, not recommand.
    fn add_reqwest_cookie(&mut self, cookie: ReqwestCookie, reqwest_url: &Url) -> &mut Self {
        self.insert_raw(&RawCookie::new(cookie.name(), cookie.value()), reqwest_url)
            .unwrap();
        self
    }
    /// May lost the metadata of cookies, not recommand.
    fn add_reqwest_cookies<'a>(
        &mut self,
        cookies: impl Iterator<Item = ReqwestCookie<'a>>,
        reqwest_url: &Url,
    ) -> &mut Self {
        cookies.for_each(|cookie| {
            self.add_reqwest_cookie(cookie, reqwest_url);
        });
        self
    }
}
