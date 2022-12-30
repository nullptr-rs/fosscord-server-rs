pub struct RegisterSchema {
    pub username: String,
    pub password: String,
    pub consent: bool,
    pub email: String,
    pub fingerprint: Option<String>,
    pub invite: Option<String>,
    pub date_of_birth: String,
    pub gift_code_sku_id: Option<String>,
    pub captcha_key: Option<String>,
    pub promotional_email_opt_in: Option<bool>
}