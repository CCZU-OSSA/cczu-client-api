use aes::Aes128Enc;
use cbc::Encryptor;
use serde::{Deserialize, Serialize};

pub type CbcAES128Enc = Encryptor<Aes128Enc>;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ElinkUserInfo {
    pub username: String,
    pub sid: String,
    #[serde(alias = "userId")]
    pub userid: String,
    #[serde(alias = "loginKey")]
    pub loginkey: String,
}
