use sea_orm::EnumIter;
use sea_orm::DeriveActiveEnum;

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Status {
    #[sea_orm(string_value = "idle")]
    Idle,
    #[sea_orm(string_value = "dnd")]
    Dnd,
    #[sea_orm(string_value = "online")]
    Online,
    #[sea_orm(string_value = "offline")]
    Offline,
    #[sea_orm(string_value = "invisible")]
    Invisible
}

pub struct ClientStatus {
    pub desktop: Option<String>,
    pub mobile: Option<String>,
    pub web: Option<String>
}