use crate::client::UserClient;

use super::app::Application;
pub struct JwcasApplication<'a> {
    client: &'a mut dyn UserClient,
    root: String,
}

impl<'a> Application<'a> for JwcasApplication<'a> {
    fn from_client(client: &'a mut dyn UserClient) -> Self {
        JwcasApplication::new(client)
    }
}

impl<'a> JwcasApplication<'a> {
    pub fn new(client: &'a mut dyn UserClient) -> Self {
        let root = client.redirect("http://219.230.159.132");
        Self { client, root }
    }
    pub fn login(&self) {
        let _api = format!("{}/web_cas/web_cas_login_jwgl.aspx", self.root);
    }
    pub fn get_class_list(&self) {
        let api = format!("{}/web_cjgl/cx_cj_xh.aspx", self.root);
        // not the same cookies, delete this after impl `login` method
        self.client.initialize_url(api.as_str());
    }

    pub fn get_grades_list(&self) {
        let _api = format!("{}/web_cjgl/cx_cj_xh.aspx", self.root);
    }
}
