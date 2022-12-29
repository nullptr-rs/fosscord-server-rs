use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct GuildDefaultConfiguration {
    #[derivative(Default(value = "250000"))]
    pub max_presences: u64,
    #[derivative(Default(value = "200"))]
    pub max_video_channel_users: u64,
    #[derivative(Default(value = "300"))]
    pub afk_timeout: u64,
    #[derivative(Default(value = "1"))]
    pub default_message_notifications: u64,
    #[derivative(Default(value = "0"))]
    pub explicit_content_filter: u64,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct UserDefaultConfiguration {
    #[derivative(Default(value = "false"))]
    pub premium: bool,
    #[derivative(Default(value = "2"))]
    pub premium_type: u64,
    #[derivative(Default(value = "true"))]
    pub verified: bool,
}
