pub mod app;
pub mod session;
pub mod simple;
pub mod types;
pub mod universal;
pub mod webvpn;

#[cfg(test)]
mod sso_test {
    use crate::sso::{app::jwcas::JwcasApplication, universal::UniversalClient};

    use super::app::{jwcas_calendar::JwcasApplicationCalendarExt, jwcas_calendar_type::Schedule};

    #[tokio::test]
    async fn test_spawn() {
        tokio::spawn(test_op());
    }
    async fn test_op() {
        loop {
            let client = UniversalClient::auto_login("user", " password")
                .await
                .unwrap();
            let app: JwcasApplication = client.visit_application();
            app.login().await.unwrap();
            app.generate_icalendar("firstweekdate".into(), Schedule::default(), None)
                .await;
        }
    }
}
