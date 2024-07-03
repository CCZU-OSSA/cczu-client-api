#[macro_export]
macro_rules! impl_auth_client {
    ($name:ident) => {
        impl_auth_client!($name, user, pwd, client, cookies);
    };
    ($name:ident,$user:ident,$pwd:ident,$client:ident,$cookies:ident) => {
        impl AuthClient for $name {
            fn get_cookies(&self) -> Arc<reqwest_cookie_store::CookieStoreMutex> {
                self.cookies.clone()
            }

            fn get_cookies_mut(&mut self) -> Arc<reqwest_cookie_store::CookieStoreMutex> {
                self.cookies.clone()
            }

            fn get_client(&self) -> std::sync::Arc<reqwest::Client> {
                self.client.clone()
            }

            fn get_client_mut(&mut self) -> std::sync::Arc<reqwest::Client> {
                self.client.clone()
            }

            fn get_user(&self) -> String {
                self.user.clone()
            }

            fn get_pwd(&self) -> String {
                self.pwd.clone()
            }
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}
    };
}
