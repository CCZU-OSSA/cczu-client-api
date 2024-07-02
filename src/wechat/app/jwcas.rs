use std::sync::Arc;

use crate::base::{
    app::Application,
    client::{AuthClient, Redirect},
};
trait SSOClient: AuthClient + Redirect {}
#[derive(Clone)]
pub struct JwcasApplication {
    _client: Arc<dyn SSOClient>,
}

impl Application for JwcasApplication {
    fn from_client(_client: Arc<dyn AuthClient>) -> Self {
        todo!()
    }
}
