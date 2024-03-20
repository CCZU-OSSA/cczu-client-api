use reqwest::Url;
use reqwest_cookie_store::CookieStore;
pub trait CopyCookies {
    fn copy_cookies(&mut self, from: &Url, to: &Url);
}

impl CopyCookies for CookieStore {
    fn copy_cookies(&mut self, from: &Url, to: &Url) {
        self.clone().matches(from).into_iter().for_each(|cookie| {
            self.insert(cookie.clone(), to).unwrap();
        });
    }
}
