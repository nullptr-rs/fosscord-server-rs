use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct KafkaBroker {
    pub ip: String,
    pub port: u64,
}
