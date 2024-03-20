use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoginCallback {
    pub username: String,
    pub sid: String,
    #[serde(alias = "userId")]
    pub userid: String,
    #[serde(alias = "loginKey")]
    pub loginkey: String,
}
