pub struct ASPSessionState {
    pub viewstate: Option<String>,
    pub viewstategenerator: Option<String>,
}

impl ASPSessionState {
    pub fn from_text(_html: String) {}
}
