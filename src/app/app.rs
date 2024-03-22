use crate::client::UserClient;

pub trait Application {
    fn from_client(client: &mut dyn UserClient) -> Self;
}
