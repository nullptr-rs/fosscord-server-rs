use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct ClientReleaseConfiguration {
    #[derivative(Default(value = "true"))]
    pub use_local_release: bool,
    #[derivative(Default(value = "String::from(\"0.0.264\")"))]
    pub upstream_version: String,
}
