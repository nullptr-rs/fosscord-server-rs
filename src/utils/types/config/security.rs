use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct CaptchaConfiguration {
    #[derivative(Default(value = "false"))]
    pub enabled: bool,
    #[derivative(Default(value = "String::from(\"recaptcha\")"))]
    pub service: String,
    pub site_key: String,
    pub secret_key: String,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct TwoFactorConfiguration {
    #[derivative(Default(value = "true"))]
    pub generate_backup_codes: bool,
}
