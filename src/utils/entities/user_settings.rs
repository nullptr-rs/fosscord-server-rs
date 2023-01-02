use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use crate::utils::interfaces::status::Status;
use crate::utils::other::snowflake::Snowflake;

#[derive(Clone, Debug, PartialEq, Default, DeriveEntityModel)]
#[sea_orm(table_name = "user_settings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(nullable, default_value = "3600")]
    pub afk_timeout: Option<i32>,
    #[sea_orm(nullable, default_value = "true")]
    pub allow_accessibility_detection: Option<bool>,
    #[sea_orm(nullable, default_value = "true")]
    pub animate_emoji: Option<bool>,
    #[sea_orm(nullable, default_value = "0")]
    pub animate_stickers: Option<i32>,
    #[sea_orm(nullable, default_value = "false")]
    pub contact_sync_enabled: Option<bool>,
    #[sea_orm(nullable, default_value = "false")]
    pub convert_emoticons: Option<bool>,
    #[sea_orm(nullable)]
    pub custom_status: Option<CustomStatus>,
    #[sea_orm(nullable, default_value = "false")]
    pub default_guilds_restricted: Option<bool>,
    #[sea_orm(nullable, default_value = "false")]
    pub detect_platform_accounts: Option<bool>,
    #[sea_orm(nullable, default_value = "true")]
    pub developer_mode: Option<bool>,
    #[sea_orm(nullable, default_value = "true")]
    pub disable_games_tab: Option<bool>,
    #[sea_orm(nullable, default_value = "false")]
    pub enable_tts_command: Option<bool>,
    #[sea_orm(nullable, default_value = "0")]
    pub explicit_content_filter: Option<i32>,
    #[sea_orm(nullable)]
    pub friend_source_flags: Option<FriendSourceFlags>,
    #[sea_orm(nullable, default_value = "false")]
    pub gateway_connected: Option<bool>,
    #[sea_orm(nullable, default_value = "false")]
    pub gif_auto_play: Option<bool>,
    //A Vec<GuildFolder> stored as a Json array
    #[sea_orm(nullable)]
    pub guild_folders: Option<Json>,
    //A Vec<String> stored as a Json array
    #[sea_orm(nullable)]
    pub guild_positions: Option<Json>,
    #[sea_orm(nullable, default_value = "true")]
    pub inline_attachment_media: Option<bool>,
    #[sea_orm(nullable, default_value = "true")]
    pub inline_embed_media: Option<bool>,
    #[sea_orm(nullable, default_value = "en-US")]
    pub locale: Option<String>,
    #[sea_orm(nullable, default_value = "false")]
    pub message_display_compact: Option<bool>,
    #[sea_orm(nullable, default_value = "true")]
    pub native_phone_integration_enabled: Option<bool>,
    #[sea_orm(nullable, default_value = "true")]
    pub render_embeds: Option<bool>,
    #[sea_orm(nullable, default_value = "true")]
    pub render_reactions: Option<bool>,
    //A Vec<String> stored as a Json array
    #[sea_orm(nullable)]
    pub restricted_guilds: Option<Json>,
    #[sea_orm(nullable, default_value = "true")]
    pub show_current_game: Option<bool>,
    #[sea_orm(nullable, default_value = "Status::Online")]
    pub status: Option<Status>,
    #[sea_orm(nullable, default_value = "false")]
    pub stream_notifications_enabled: Option<bool>,
    #[sea_orm(nullable, default_value = "dark")]
    pub theme: Option<String>,
    #[sea_orm(nullable, default_value = "0")]
    pub timezone_offset: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct CustomStatus {
    pub emoji_id: Option<Snowflake>,
    pub emoji_name: Option<String>,
    pub expires_at: Option<DateTime>,
    pub text: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct GuildFolder {
    pub color: Option<i32>,
    pub guild_ids: Vec<Snowflake>,
    pub id: Snowflake,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct FriendSourceFlags {
    pub all: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::Id",
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