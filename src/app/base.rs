use crate::client::UserClient;

pub trait Application<'a> {
    fn from_client(client: &'a mut dyn UserClient) -> Self;
}
