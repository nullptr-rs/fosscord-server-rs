use crate::utils::entities::application::Relation;
use crate::utils::types::snowflake::Snowflake;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Snowflake,
}

impl ActiveModelBehavior for ActiveModel {}
