use aes::Aes128Enc;
use cbc::Encryptor;
use serde::{Deserialize, Serialize};

pub type CbcAES128Enc = Encryptor<Aes128Enc>;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ElinkLoginInfo {
    pub username: Option<String>,
    pub sid: Option<String>,
    #[serde(alias = "userId")]
    pub userid: Option<String>,
    #[serde(alias = "loginKey")]
    pub loginkey: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ElinkGroupInfo {
    pub name: Option<String>,
    pub id: Option<String>,
    pub description: Option<String>,
    pub creator: Option<String>,
    #[serde(alias = "createTime")]
    pub createtime: Option<String>, // 格式: yyyy-MM-dd HH:mm:ss
    pub updator: Option<String>,
    #[serde(alias = "authTypeId")]
    pub auth_type_id: Option<String>,

    #[serde(alias = "updateTime")]
    pub updatetime: Option<String>, // 格式: yyyy-MM-dd HH:mm:ss
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ElinkUserInfoData {
    pub username: Option<String>,
    pub name: Option<String>,
    pub id: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    #[serde(alias = "userState")]
    pub user_state: Option<String>,
    #[serde(alias = "lastLoginTime")]
    pub lastlogintime: Option<String>, // 格式: yyyy-MM-dd HH:mm:ss
    #[serde(alias = "userGroups")]
    pub user_groups: Option<Vec<ElinkGroupInfo>>,
    pub creator: Option<String>,
    #[serde(alias = "createTime")]
    pub createtime: Option<String>, // 格式: yyyy-MM-dd HH:mm:ss
    pub updator: Option<String>,
    #[serde(alias = "updateTime")]
    pub updatetime: Option<String>, // 格式: yyyy-MM-dd HH:mm:ss
    #[serde(alias = "dingNickName")]
    pub ding_nick_name: Option<String>,
    #[serde(alias = "qyWeChatUserId")]
    pub qy_we_chat_user_id: Option<String>,
    #[serde(alias = "weChatNickName")]
    pub we_chat_nick_name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ElinkUserInfo {
    pub code: Option<String>,
    pub messages: Option<String>,
    pub data: Option<ElinkUserInfoData>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ElinkServiceInfo {
    pub code: Option<String>,
    pub messages: Option<String>,
    pub data: Option<ElinkServiceInfoData>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ElinkUserServiceInfo {
    pub code: Option<String>,
    pub messages: Option<String>,
    pub data: Option<Vec<ElinkServiceData>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ElinkServiceInfoData {
    pub title: Option<String>,
    pub key: Option<String>,
    pub children: Option<Vec<ElinkServiceInfoData>>,
    #[serde(alias = "serviceList")]
    pub service_list: Option<Vec<ElinkServiceData>>,
    #[serde(alias = "serviceAllList")]
    pub service_all_list: Option<Vec<ElinkServiceData>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ElinkServiceGatewayData {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(alias = "uniqueNo")]
    pub unique_no: Option<String>,
    pub server: Option<String>,
    pub description: Option<String>,
    #[serde(alias = "type")]
    pub type_of: Option<String>,
    #[serde(alias = "adminAddr")]
    pub admin_addr: Option<String>,
    #[serde(alias = "nginxPort")]
    pub nginx_port: Option<String>,
    #[serde(alias = "connectState")]
    pub connect_state: Option<String>,
    #[serde(alias = "publicServer")]
    pub public_server: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ElinkServiceData {
    pub id: Option<String>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub server: Option<String>,
    pub description: Option<String>,
    #[serde(alias = "type")]
    pub type_of: Option<String>,
    #[serde(alias = "urlPlus")]
    pub url_plus: Option<String>,
    #[serde(alias = "hostMd5")]
    pub host_md5: Option<String>,
    #[serde(alias = "gatewayVo")]
    pub gateway_vo: Option<ElinkServiceGatewayData>,
}
