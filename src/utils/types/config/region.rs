use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct Region {
    pub id: String,
    pub name: String,
    pub endpoint: String,
    pub location: Location,
    pub vip: bool,
    pub custom: bool,
    pub deprecated: bool,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}
