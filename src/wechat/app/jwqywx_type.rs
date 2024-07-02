use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Message<T> {
    pub status: i32,
    pub message: Vec<T>,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginUserData {
    #[serde(alias = "yhdm")]
    pub userid: String,
    #[serde(alias = "yhmc")]
    pub username: String,
    #[serde(alias = "yhsf")]
    pub userident: String,
    #[serde(alias = "xq")]
    pub term: String,
    #[serde(alias = "dqz")]
    pub current_value: i32,
    #[serde(alias = "zc")]
    pub position: i32,
    #[serde(alias = "gh")]
    pub employee_number: i32,
    pub smscode: i32,
    #[serde(alias = "xb")]
    pub gender: String,
    #[serde(alias = "yhqx")]
    pub permission: String,
}
