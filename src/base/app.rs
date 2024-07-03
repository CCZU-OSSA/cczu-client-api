use std::sync::Arc;

use super::client::{AuthClient, SSOClient};

pub trait Application {
    fn from_client(client: Arc<dyn AuthClient + Send + Sync>) -> Self;
}

pub trait SSOApplication {
    fn from_client(client: Arc<dyn SSOClient + Send + Sync>) -> Self;
}
