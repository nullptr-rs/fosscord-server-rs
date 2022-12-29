use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct DateOfBirthConfiguration {
    #[derivative(Default(value = "true"))]
    pub required: bool,
    #[derivative(Default(value = "13"))]
    pub min_age: u64,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct EmailConfiguration {
    #[derivative(Default(value = "false"))]
    pub required: bool,
    #[derivative(Default(value = "false"))]
    pub allow_list: bool,
    #[derivative(Default(value = "true"))]
    pub block_list: bool,
    pub block_list_domains: Vec<String>,
}

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct PasswordConfiguration {
    #[derivative(Default(value = "false"))]
    pub required: bool,
    #[derivative(Default(value = "8"))]
    pub min_length: u64,
    #[derivative(Default(value = "2"))]
    pub min_numbers: u64,
    #[derivative(Default(value = "2"))]
    pub min_uppercase: u64,
    pub min_symbols: u64,
}
