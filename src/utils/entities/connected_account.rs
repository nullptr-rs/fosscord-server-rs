use crate::utils::other::snowflake::Snowflake;
use sea_orm::prelude::*;

pub struct PublicConnectedAccount {
    pub name: String,
    pub _type: String,
    pub verified: bool,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "connected_accounts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(nullable)]
    pub user_id: Option<Snowflake>,
    pub access_token: String,
    pub friend_sync: bool,
    pub name: String,
    pub revoked: bool,
    pub show_activity: bool,
    pub _type: String,
    pub verified: bool,
    pub visibility: i32,
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

impl From<Model> for PublicConnectedAccount {
    fn from(value: Model) -> Self {
        Self {
            name: value.name,
            _type: value._type,
            verified: value.verified,
        }
    }
}
