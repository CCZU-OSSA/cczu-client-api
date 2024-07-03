use aes::Aes128Enc;
use cbc::Encryptor;
use reqwest::Response;
use serde::Deserialize;

pub type CbcAES128Enc = Encryptor<Aes128Enc>;

#[derive(Deserialize, Debug, Clone)]
pub struct ElinkLoginInfo {
    pub username: Option<String>,
    pub sid: Option<String>,
    #[serde(rename = "userId")]
    pub userid: Option<String>,
    #[serde(rename = "loginKey")]
    pub loginkey: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ElinkGroupInfo {
    pub name: Option<String>,
    pub id: Option<String>,
    pub description: Option<String>,
    pub creator: Option<String>,
    #[serde(rename = "createTime")]
    pub createtime: Option<String>, // 格式: yyyy-MM-dd HH:mm:ss
    pub updator: Option<String>,
    #[serde(rename = "authTypeId")]
    pub auth_type_id: Option<String>,

    #[serde(rename = "updateTime")]
    pub updatetime: Option<String>, // 格式: yyyy-MM-dd HH:mm:ss
}

#[derive(Deserialize, Debug, Clone)]
pub struct ElinkUserInfoData {
    pub username: Option<String>,
    pub name: Option<String>,
    pub id: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    #[serde(rename = "userState")]
    pub user_state: Option<String>,
    #[serde(rename = "lastLoginTime")]
    pub lastlogintime: Option<String>, // 格式: yyyy-MM-dd HH:mm:ss
    #[serde(rename = "userGroups")]
    pub user_groups: Option<Vec<ElinkGroupInfo>>,
    pub creator: Option<String>,
    #[serde(rename = "createTime")]
    pub createtime: Option<String>, // 格式: yyyy-MM-dd HH:mm:ss
    pub updator: Option<String>,
    #[serde(rename = "updateTime")]
    pub updatetime: Option<String>, // 格式: yyyy-MM-dd HH:mm:ss
    #[serde(rename = "dingNickName")]
    pub ding_nick_name: Option<String>,
    #[serde(rename = "qyWeChatUserId")]
    pub qy_we_chat_user_id: Option<String>,
    #[serde(rename = "weChatNickName")]
    pub we_chat_nick_name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ElinkUserInfo {
    pub code: Option<String>,
    pub messages: Option<String>,
    pub data: Option<ElinkUserInfoData>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ElinkServiceInfo {
    pub code: Option<String>,
    pub messages: Option<String>,
    pub data: Option<ElinkServiceInfoData>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ElinkUserServiceInfo {
    pub code: Option<String>,
    pub messages: Option<String>,
    pub data: Option<Vec<ElinkServiceData>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ElinkServiceInfoData {
    pub title: Option<String>,
    pub key: Option<String>,
    pub children: Option<Vec<ElinkServiceInfoData>>,
    #[serde(rename = "serviceList")]
    pub service_list: Option<Vec<ElinkServiceData>>,
    #[serde(rename = "serviceAllList")]
    pub service_all_list: Option<Vec<ElinkServiceData>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ElinkServiceGatewayData {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "uniqueNo")]
    pub unique_no: Option<String>,
    pub server: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_of: Option<String>,
    #[serde(rename = "adminAddr")]
    pub admin_addr: Option<String>,
    #[serde(rename = "nginxPort")]
    pub nginx_port: Option<String>,
    #[serde(rename = "connectState")]
    pub connect_state: Option<String>,
    #[serde(rename = "publicServer")]
    pub public_server: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ElinkServiceData {
    pub id: Option<String>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub server: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_of: Option<String>,
    #[serde(rename = "urlPlus")]
    pub url_plus: Option<String>,
    #[serde(rename = "hostMd5")]
    pub host_md5: Option<String>,
    #[serde(rename = "gatewayVo")]
    pub gateway_vo: Option<ElinkServiceGatewayData>,
}

pub enum LoginConnectType {
    WEBVPN,
    COMMON,
}

pub struct UniversalSSOLogin {
    pub response: Response,
    pub login_connect_type: LoginConnectType,
}
