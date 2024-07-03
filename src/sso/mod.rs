pub mod app;
pub mod session;
pub mod simple;
pub mod types;
pub mod universal;
pub mod webvpn;

#[cfg(test)]
mod sso_test {
    use crate::sso::{app::jwcas::JwcasApplication, universal::UniversalClient};

    #[tokio::test]
    async fn test_spawn() {
        tokio::spawn(test_op());
    }
    
    async fn test_op() -> Option<()> {
        let client = UniversalClient::auto("user", " password").await;
        let app = client.visit_application::<JwcasApplication>();
        app.login().await.unwrap();
        client.visit_application::<JwcasApplication>();
        None
    }
}
