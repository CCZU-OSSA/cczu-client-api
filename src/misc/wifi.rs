use regex::Regex;

pub async fn login_wifi(user: String, pwd: String) -> Result<String, String> {
    if let Ok(response) = reqwest::get("http://6.6.6.6/").await {
        if let Ok(text) = response.text().await {
            let re = Regex::new(r#"wlanacip=(.*?)&ssid"#).unwrap();
            if let Some(raw_acip) = re.find(&text) {
                let acip = raw_acip
                    .as_str()
                    .trim_start_matches("wlanacip=")
                    .trim_end_matches("&ssid");
                if let Ok(response) = reqwest::get(format!(
                "http://172.16.1.52:801/eportal/portal/login?wlan_ac_ip={}&user_account={}&user_password={}",acip, user, pwd)).await{
                    if let Ok(callback) = response.text().await {
                        return Ok(callback);
                    }
                } else {
                    return Err("连接认证失败错误".into());
                }
            } else {
                return Err("获取ACIP错误".into());
            }
        }
    }

    Err("获取页面错误".into())
}
