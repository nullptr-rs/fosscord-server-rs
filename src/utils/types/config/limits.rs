use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct ChannelLimitsConfiguration {
    #[derivative(Default(value = "500"))]
    pub max_pins: u64,
    #[derivative(Default(value = "1024"))]
    pub max_topics: u64,
    #[derivative(Default(value = "100"))]
    pub max_webhooks: u64,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct GlobalRateLimitsConfiguration {
    pub register: GlobalRateLimitConfiguration,
    pub send_message: GlobalRateLimitConfiguration,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct GlobalRateLimitConfiguration {
    #[derivative(Default(value = "100"))]
    pub limit: u64,
    #[derivative(Default(value = "60 * 60 * 1000"))]
    pub window: u64,
    #[derivative(Default(value = "true"))]
    pub enabled: bool,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct GuildLimitsConfiguration {
    #[derivative(Default(value = "1000"))]
    pub max_roles: u64,
    #[derivative(Default(value = "2000"))]
    pub max_emojis: u64,
    #[derivative(Default(value = "25000000"))]
    pub max_members: u64,
    #[derivative(Default(value = "65535"))]
    pub max_channels: u64,
    #[derivative(Default(value = "65535"))]
    pub max_channels_per_category: u64,
    #[derivative(Default(value = "3"))]
    pub hide_offline_members: u64,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct MessageLimitsConfiguration {
    #[derivative(Default(value = "1048576"))]
    pub max_characters: u64,
    #[derivative(Default(value = "160"))]
    pub max_tts_characters: u64,
    #[derivative(Default(value = "2048"))]
    pub max_reactions: u64,
    #[derivative(Default(value = "1024 * 1024 * 1024"))]
    pub max_attachments_size: u64,
    #[derivative(Default(value = "1000"))]
    pub max_bulk_delete: u64,
    #[derivative(Default(value = "1024 * 1024 * 5"))]
    pub max_embed_download_size: u64,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct RateLimitsConfiguration {
    #[derivative(Default(value = "true"))]
    pub disabled: bool,
    pub ip: RateLimitOptions,
    pub global: RateLimitOptions,
    pub error: RateLimitOptions,
    pub routes: RouteLimitsConfiguration,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct UserLimitsConfiguration {
    #[derivative(Default(value = "1048576"))]
    pub max_guilds: u64,
    #[derivative(Default(value = "127"))]
    pub max_username: u64,
    #[derivative(Default(value = "5000"))]
    pub max_friends: u64,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct AuthRateLimit {
    pub login: RateLimitOptions,
    pub register: RateLimitOptions,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct RouteLimitsConfiguration {
    pub guild: RateLimitOptions,
    pub webhook: RateLimitOptions,
    pub channel: RateLimitOptions,
    pub auth: AuthRateLimit,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct RateLimitOptions {
    pub bot: u64,
    pub count: u64,
    pub window: u64,
    pub only_ip: bool,
}
