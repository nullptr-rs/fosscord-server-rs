use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct AutoJoinConfiguration {
    #[derivative(Default(value = "true"))]
    pub enabled: bool,
    pub guilds: Vec<String>,
    #[derivative(Default(value = "true"))]
    pub can_leave: bool,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct DiscoveryConfiguration {
    #[derivative(Default(value = "false"))]
    pub show_all_guilds: bool,
    #[derivative(Default(value = "false"))]
    pub use_recommendations: bool,
    #[derivative(Default(value = "0"))]
    pub offset: u64,
    #[derivative(Default(value = "24"))]
    pub limit: u64,
}
