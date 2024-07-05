pub mod jwqywx;
pub mod jwqywx_type;

#[cfg(test)]
mod jwqywx_test {
    use crate::{
        base::client::{SimpleClient, Visitor},
        wechat::app::jwqywx::JwqywxApplication,
    };
    #[tokio::test]
    async fn test() {
        let client = SimpleClient::new("user", "pwd");
        let app = client.visit_application::<JwqywxApplication>();
        app.login().await.unwrap();
        dbg!(app.get_grades().await.unwrap());
    }
}
