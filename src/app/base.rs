use std::sync::Arc;

use crate::client::UserClient;

pub trait Application {
    fn from_client(client: Arc<dyn UserClient + Send + Sync>) -> Self;
}
