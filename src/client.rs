use reqwest::Client;

pub trait UserClient {
    fn login(&self);
    fn get_client(&self) -> &Client;
    fn get_client_mut(&mut self) -> &mut Client;
}
