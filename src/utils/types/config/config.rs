use crate::utils::types::config::client::ClientReleaseConfiguration;
use crate::utils::types::config::default::{GuildDefaultConfiguration, UserDefaultConfiguration};
use crate::utils::types::config::guild::{AutoJoinConfiguration, DiscoveryConfiguration};
use crate::utils::types::config::kafka::KafkaBroker;
use crate::utils::types::config::limits::{
    ChannelLimitsConfiguration, GlobalRateLimitConfiguration, GuildLimitsConfiguration,
    MessageLimitsConfiguration, RateLimitsConfiguration, UserLimitsConfiguration,
};
use crate::utils::types::config::region::Region;
use crate::utils::types::config::register::{
    DateOfBirthConfiguration, EmailConfiguration, PasswordConfiguration,
};
use crate::utils::types::config::security::{CaptchaConfiguration, TwoFactorConfiguration};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct ApiConfiguration {
    #[derivative(Default(value = "String::from(\"9\")"))]
    pub default_version: String,
    #[derivative(Default(
        value = "vec![String::from(\"6\"), String::from(\"7\"), String::from(\"8\"), String::from(\"9\")]"
    ))]
    pub active_versions: Vec<String>,
    #[derivative(Default(value = "true"))]
    pub use_fosscord_enhancements: bool,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct ClientConfiguration {
    #[derivative(Default(value = "ClientReleaseConfiguration::new()"))]
    pub releases: ClientReleaseConfiguration,
    #[derivative(Default(value = "true"))]
    pub use_test_client: bool,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct DefaultConfiguration {
    #[derivative(Default(value = "GuildDefaultConfiguration::new()"))]
    pub guild: GuildDefaultConfiguration,
    #[derivative(Default(value = "UserDefaultConfiguration::new()"))]
    pub user: UserDefaultConfiguration,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct EndpointConfiguration {
    pub private_endpoint: String,
    pub public_endpoint: String,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct GeneralConfiguration {
    #[derivative(Default(value = "String::from(\"Fosscord instance\")"))]
    pub instance_name: String,
    #[derivative(Default(
        value = "String::from(\"This is a Fosscord instance made in the pre-release days\")"
    ))]
    pub instance_description: String,
    #[derivative(Default(value = "String::from(\"http://localhost:3001\")"))]
    pub public_url: String,
    pub front_page: String,
    pub tos_page: String,
    #[derivative(Default(value = "String::from(\"noreply@localhost.local\")"))]
    pub correspondence_email: String,
    pub correspondence_user_id: String,
    pub image: String,
    pub instance_id: String,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct GifConfiguration {
    #[derivative(Default(value = "true"))]
    pub enabled: bool,
    #[derivative(Default(value = "String::from(\"tenor\")"))]
    pub provider: String,
    #[derivative(Default(value = "String::from(\"LIVDSRZULELA\")"))]
    pub api_key: String,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct GuildConfiguration {
    #[derivative(Default(value = "DiscoveryConfiguration::new()"))]
    pub discovery: DiscoveryConfiguration,
    #[derivative(Default(value = "AutoJoinConfiguration::new()"))]
    pub auto_join: AutoJoinConfiguration,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct KafkaConfiguration {
    pub brokers: Vec<KafkaBroker>,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct LimitsConfiguration {
    #[derivative(Default(value = "UserLimitsConfiguration::new()"))]
    pub user: UserLimitsConfiguration,
    #[derivative(Default(value = "GuildLimitsConfiguration::new()"))]
    pub guild: GuildLimitsConfiguration,
    #[derivative(Default(value = "MessageLimitsConfiguration::new()"))]
    pub message: MessageLimitsConfiguration,
    #[derivative(Default(value = "ChannelLimitsConfiguration::new()"))]
    pub channel: ChannelLimitsConfiguration,
    #[derivative(Default(value = "RateLimitsConfiguration::new()"))]
    pub rate: RateLimitsConfiguration,
    #[derivative(Default(value = "GlobalRateLimitConfiguration::new()"))]
    pub global_rate: GlobalRateLimitConfiguration,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct LoginConfiguration {
    #[derivative(Default(value = "false"))]
    pub require_captcha: bool,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct MetricsConfiguration {
    #[derivative(Default(value = "30000"))]
    pub timeout: u64,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct RabbitMQConfiguration {
    pub host: String,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct RegionConfiguration {
    #[derivative(Default(value = "String::from(\"fosscord\")"))]
    pub default: String,
    #[derivative(Default(value = "true"))]
    pub use_default_as_optimal: bool,
    pub available: Vec<Region>,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct RegisterConfiguration {
    #[derivative(Default(value = "EmailConfiguration::new()"))]
    pub email: EmailConfiguration,
    #[derivative(Default(value = "DateOfBirthConfiguration::new()"))]
    pub date_of_birth: DateOfBirthConfiguration,
    #[derivative(Default(value = "PasswordConfiguration::new()"))]
    pub password: PasswordConfiguration,
    #[derivative(Default(value = "false"))]
    pub disabled: bool,
    #[derivative(Default(value = "true"))]
    pub require_captcha: bool,
    #[derivative(Default(value = "false"))]
    pub require_invite: bool,
    #[derivative(Default(value = "true"))]
    pub allow_guests: bool,
    #[derivative(Default(value = "true"))]
    pub guests_require_invite: bool,
    #[derivative(Default(value = "true"))]
    pub allow_new_registration: bool,
    #[derivative(Default(value = "true"))]
    pub block_proxies: bool,
    #[derivative(Default(value = "false"))]
    pub incrementing_discriminators: bool,
    #[derivative(Default(value = "String::from(\"0\")"))]
    pub default_rights: String,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct SecurityConfiguration {
    #[derivative(Default(value = "CaptchaConfiguration::new()"))]
    pub captcha: CaptchaConfiguration,
    #[derivative(Default(value = "TwoFactorConfiguration::new()"))]
    pub two_factor: TwoFactorConfiguration,
    #[derivative(Default(value = "true"))]
    pub auto_update: bool,
    pub request_signature: String,
    pub jwt_secret: String,
    pub forwarded_for: String,
    #[derivative(Default(
        value = "String::from(\"eca677b284b3bac29eb72f5e496aa9047f26543605efe99ff2ce35c9\")"
    ))]
    pub ipdata_api_key: String,
    #[derivative(Default(value = "10"))]
    pub mfa_backup_code_count: u64,
    #[derivative(Default(value = "4"))]
    pub mfa_backup_code_bytes: u64,
    #[derivative(Default(value = "true"))]
    pub stats_world_readable: bool,
    #[derivative(Default(value = "1000 * 60 * 60 * 24 * 7"))]
    pub default_registration_token_expiration: u64,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct SentryConfiguration {
    #[derivative(Default(value = "false"))]
    pub enabled: bool,
    #[derivative(Default(
        value = "String::from(\"https://05e8e3d005f34b7d97e920ae5870a5e5@sentry.thearcanebrony.net/6\")"
    ))]
    pub endpoint: String,
    #[derivative(Default(value = "1.0"))]
    pub traces_sample_rate: f64,
    pub environment: String,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct TemplateConfiguration {
    #[derivative(Default(value = "true"))]
    pub enabled: bool,
    #[derivative(Default(value = "true"))]
    pub allow_template_creation: bool,
    #[derivative(Default(value = "true"))]
    pub allow_discord_templates: bool,
    #[derivative(Default(value = "true"))]
    pub allow_raws: bool,
}
