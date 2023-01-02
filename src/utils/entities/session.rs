use sea_orm::{Related, RelationDef};
use crate::utils::other::snowflake::Snowflake;
use sea_orm::entity::prelude::*;
use crate::utils::interfaces::status::Status;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(nullable)]
    pub user_id: Option<Snowflake>,
    pub session_id: String,
    //A Vec<Activity> stored as a Json array
    #[sea_orm(nullable)]
    pub activities: Option<Json>,
    pub status: Status,
}

pub struct ClientInfo {
    pub client: String,
    pub os: String,
    pub version: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}