use crate::utils::other::snowflake::Snowflake;
use sea_orm::prelude::*;

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum RelationshipType {
    #[sea_orm(num_value = 4)]
    Outgoing = 4,
    #[sea_orm(num_value = 3)]
    Incoming = 3,
    #[sea_orm(num_value = 2)]
    Blocked = 2,
    #[sea_orm(num_value = 1)]
    Friends = 1,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "relationships")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(indexed, unique)]
    pub from_id: Snowflake,
    #[sea_orm(indexed, unique)]
    pub to_id: Snowflake,
    #[sea_orm(nullable)]
    pub nickname: Option<String>,
    pub _type: RelationshipType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::FromId",
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
