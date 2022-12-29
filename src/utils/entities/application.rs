use crate::utils::types::snowflake::Snowflake;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "applications")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Snowflake,
    pub name: String,
    #[sea_orm(nullable)]
    pub icon: Option<String>,
    #[sea_orm(nullable)]
    pub description: Option<String>,
    #[sea_orm(nullable)]
    pub summary: Option<String>,
    #[sea_orm(nullable)]
    pub _type: Option<Json>,
    #[sea_orm(default_value = "true")]
    pub hook: bool,
    #[sea_orm(default_value = "true")]
    pub bot_public: bool,
    #[sea_orm(default_value = "false")]
    pub bot_require_code_grant: bool,
    pub verify_key: String,
    pub owner_id: Snowflake,
    #[sea_orm(default_value = "0")]
    pub flags: u64,
    //A Vec<String> stored as a Json array
    #[sea_orm(nullable)]
    pub redirect_uris: Option<Json>,
    #[sea_orm(nullable, default_value = "0")]
    pub rpc_application_state: Option<u64>,
    #[sea_orm(nullable, default_value = "1")]
    pub store_application_state: Option<u64>,
    #[sea_orm(nullable, default_value = "1")]
    pub verification_state: Option<u64>,
    #[sea_orm(nullable)]
    pub interactions_endpoint_url: Option<String>,
    #[sea_orm(nullable, default_value = "true")]
    pub integration_public: Option<bool>,
    #[sea_orm(nullable, default_value = "false")]
    pub integration_require_code_grant: Option<bool>,
    #[sea_orm(nullable, default_value = "1")]
    pub discoverability_state: Option<u64>,
    #[sea_orm(nullable, default_value = "2240")]
    pub discovery_eligibility_flags: Option<u64>,
    pub bot_user_id: Snowflake,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::OwnerId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
