use crate::utils::types::config::config::{
    ApiConfiguration, ClientConfiguration, DefaultConfiguration, EndpointConfiguration,
    GeneralConfiguration, GifConfiguration, GuildConfiguration, KafkaConfiguration,
    LimitsConfiguration, LoginConfiguration, MetricsConfiguration, RabbitMQConfiguration,
    RegionConfiguration, RegisterConfiguration, SecurityConfiguration, SentryConfiguration,
    TemplateConfiguration,
};

pub struct Configuration {
    pub gateway: EndpointConfiguration,
    pub cdn: EndpointConfiguration,
    pub api: ApiConfiguration,
    pub general: GeneralConfiguration,
    pub limits: LimitsConfiguration,
    pub security: SecurityConfiguration,
    pub login: LoginConfiguration,
    pub register: RegisterConfiguration,
    pub regions: RegionConfiguration,
    pub guild: GuildConfiguration,
    pub gif: GifConfiguration,
    pub rabbitmq: RabbitMQConfiguration,
    pub kafka: KafkaConfiguration,
    pub templates: TemplateConfiguration,
    pub client: ClientConfiguration,
    pub metrics: MetricsConfiguration,
    pub sentry: SentryConfiguration,
    pub defaults: DefaultConfiguration,
}

impl Configuration {
    pub fn new() -> Configuration {
        let gateway = EndpointConfiguration {
            private_endpoint: String::from("wss://localhost:3000"),
            public_endpoint: String::from("ws://localhost:3001"),
        };
        let cdn = EndpointConfiguration {
            private_endpoint: String::from("https://localhost:3000"),
            public_endpoint: String::from("http://localhost:3001"),
        };

        Configuration {
            gateway,
            cdn,
            api: ApiConfiguration::new(),
            general: GeneralConfiguration::new(),
            limits: LimitsConfiguration::new(),
            security: SecurityConfiguration::new(),
            login: LoginConfiguration::new(),
            register: RegisterConfiguration::new(),
            regions: RegionConfiguration::new(),
            guild: GuildConfiguration::new(),
            gif: GifConfiguration::new(),
            rabbitmq: RabbitMQConfiguration::new(),
            kafka: KafkaConfiguration::new(),
            templates: TemplateConfiguration::new(),
            client: ClientConfiguration::new(),
            metrics: MetricsConfiguration::new(),
            sentry: SentryConfiguration::new(),
            defaults: DefaultConfiguration::new(),
        }
    }
}
