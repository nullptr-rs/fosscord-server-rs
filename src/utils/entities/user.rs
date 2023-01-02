use crate::shared::{CONFIGURATION, SNOWFLAKE_GENERATOR};
use crate::utils::config::Configuration;
use crate::utils::other::snowflake::Snowflake;
use crate::utils::other::string;
use crate::utils::schemas::auth_schemas::RegisterSchema;
use rand::Rng;
use sea_orm::entity::prelude::*;
use sea_orm::QuerySelect;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::error::Error;
use std::sync::Arc;

pub const DISCORD_EMPLOYEE_FLAG: u64 = 1 << 0;
pub const PARTNER_SERVER_OWNER_FLAG: u64 = 1 << 1;
pub const HYPESQUAD_EVENTS_FLAG: u64 = 1 << 2;
pub const BUG_HUNTER_LEVEL_1_FLAG: u64 = 1 << 3;
pub const MFA_SMS_FLAG: u64 = 1 << 4;
pub const PREMIUM_PROMO_DISMISSED_FLAG: u64 = 1 << 5;
pub const HOUSE_BRAVERY_FLAG: u64 = 1 << 6;
pub const HOUSE_BRILLIANCE_FLAG: u64 = 1 << 7;
pub const HOUSE_BALANCE_FLAG: u64 = 1 << 8;
pub const EARLY_SUPPORTER_FLAG: u64 = 1 << 9;
pub const TEAM_USER_FLAG: u64 = 1 << 10;
pub const TRUST_AND_SAFETY_FLAG: u64 = 1 << 11;
pub const SYSTEM_FLAG: u64 = 1 << 12;
pub const HAS_UNREAD_URGENT_MESSAGES_FLAG: u64 = 1 << 13;
pub const BUG_HUNTER_LEVEL_2_FLAG: u64 = 1 << 14;
pub const UNDERAGE_DELETED_FLAG: u64 = 1 << 15;
pub const VERIFIED_BOT_FLAG: u64 = 1 << 16;
pub const EARLY_VERIFIED_BOT_DEVELOPER_FLAG: u64 = 1 << 17;
pub const CERTIFIED_MODERATOR_FLAG: u64 = 1 << 18;
pub const BOT_HTTP_INTERACTIONS_FLAG: u64 = 1 << 19;

pub struct PublicUser {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    pub public_flags: i32,
    pub avatar: Option<String>,
    pub accent_color: Option<i32>,
    pub banner: Option<String>,
    pub bio: Option<String>,
    pub bot: bool,
    pub premium_since: Option<DateTime>,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    #[sea_orm(nullable)]
    pub avatar: Option<String>,
    #[sea_orm(nullable)]
    pub accent_color: Option<i32>,
    #[sea_orm(nullable)]
    pub banner: Option<String>,
    #[sea_orm(nullable)]
    pub phone: Option<String>,
    #[sea_orm(default_value = "false")]
    pub desktop: bool,
    #[sea_orm(default_value = "false")]
    pub mobile: bool,
    #[sea_orm(default_value = "false")]
    pub premium: bool,
    #[sea_orm(default_value = "2")]
    pub premium_type: i32,
    #[sea_orm(default_value = "false")]
    pub bot: bool,
    #[sea_orm(nullable)]
    pub bio: Option<String>,
    #[sea_orm(default_value = "false")]
    pub system: bool,
    #[sea_orm(default_value = "true")]
    pub nsfw_allowed: bool,
    #[sea_orm(nullable)]
    pub mfa_enabled: Option<bool>,
    #[sea_orm(nullable)]
    pub totp_secret: Option<String>,
    #[sea_orm(nullable)]
    pub totp_last_ticket: Option<String>,
    #[sea_orm(default_value = "chrono::Utc::now().into()")]
    pub created_at: DateTime,
    #[sea_orm(nullable)]
    pub premium_since: Option<DateTime>,
    #[sea_orm(default_value = "true")]
    pub verified: bool,
    #[sea_orm(default_value = "false")]
    pub disabled: bool,
    #[sea_orm(default_value = "false")]
    pub deleted: bool,
    #[sea_orm(nullable)]
    pub email: Option<String>,
    #[sea_orm(default_value = "String::from(\"0\")")]
    pub flags: String,
    #[sea_orm(default_value = "0")]
    pub public_flags: i32,
    #[sea_orm(column_type = "BigInteger", default_value = "String::from(\"0\")")]
    pub rights: String,
    pub data: Data,
    //A Vec<String> stored as a Json array
    pub fingerprints: Json,
    pub extended_settings: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::session::Entity")]
    Session,
    #[sea_orm(has_many = "super::relationship::Entity")]
    Relationship,
    #[sea_orm(has_many = "super::connected_account::Entity")]
    ConnectedAccount,
    #[sea_orm(has_one = "super::user_settings::Entity")]
    Settings,
}

#[derive(Serialize, Deserialize, FromJsonQueryResult)]
pub struct Data {
    pub valid_tokens_since: DateTime,
    pub hash: Option<String>,
}

impl Model {
    pub fn set_discriminator(&mut self, discriminator: String) -> Result<(), Box<dyn Error>> {
        let parsed_discriminator = discriminator.parse::<u64>()?;

        if parsed_discriminator >= 10000 {
            return Err("Discriminator must be less or equal than 9999".into());
        }

        self.discriminator = format!("{:0>4}", parsed_discriminator);
        Ok(())
    }
}

pub async fn generate_discriminator(
    username: String,
    database: Arc<DatabaseConnection>,
) -> Result<String, Box<dyn Error>> {
    let configuration: &Configuration = CONFIGURATION.get().unwrap();

    if configuration.register.incrementing_discriminators {
        let users = Entity::find()
            .filter(Column::Username.eq(username.clone()))
            .all(database.as_ref())
            .await?;
        let highest_discriminator = users
            .iter()
            .map(|user| user.discriminator.parse::<u64>().unwrap())
            .max()
            .unwrap_or(0);

        let discriminator = highest_discriminator + 1;
        if discriminator >= 10000 {
            return Err("Discriminator must be less or equal than 9999".into());
        }

        Ok(format!("{:0>4}", discriminator))
    } else {
        let mut tries = 0;
        loop {
            tries += 1;
            if tries >= 5 {
                return Err("Failed to generate a unique discriminator".into());
            }

            let discriminator = format!("{:0>4}", rand::thread_rng().gen_range(0..=9999));
            let exists = Entity::find()
                .filter(Column::Username.eq(username.clone()))
                .filter(Column::Discriminator.eq(discriminator.clone()))
                .select_only()
                .column(Column::Id)
                .one(database.as_ref())
                .await?;

            if exists.is_none() {
                return Ok(discriminator);
            }
        }
    }
}

pub async fn register(
    schema: RegisterSchema,
    locale: String,
    database: Arc<DatabaseConnection>,
) -> Result<PublicUser, Box<dyn Error>> {
    let username = string::trim_regex(&schema.username, string::SPECIAL_CHAR);
    let discriminator = generate_discriminator(username.clone(), database.clone()).await?;
    let snowflake = SNOWFLAKE_GENERATOR.get().unwrap().generate_snowflake();

    let data = Data {
        hash: Some(schema.password),
        valid_tokens_since: chrono::Utc::now().into(),
    };
    let settings = super::user_settings::Model {
        locale: Some(locale),
        ..Default::default()
    };
    let user = Model {
        id: snowflake.clone(),
        username,
        discriminator,
        email: Some(schema.email),
        data,
        settings,
        ..Default::default()
    };

    user.save(database).await?;

    tokio::spawn(async move {
        let configuration = CONFIGURATION.get().unwrap();

        if configuration.guild.auto_join.enabled {
            for guild in configuration.guild.auto_join.guilds {
                //TODO: Add guild join
                //Member::add_to_guild(snowflake.clone(), guild, database.clone()).await;
            }
        }
    });

    Ok(user.into())
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl Related<super::relationship::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Relationship.def()
    }
}

impl Related<super::connected_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConnectedAccount.def()
    }
}

impl Related<super::user_settings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserSettings.def()
    }
}

impl From<Model> for PublicUser {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            username: value.username,
            discriminator: value.discriminator,
            public_flags: value.public_flags,
            avatar: value.avatar,
            accent_color: value.accent_color,
            banner: value.banner,
            bio: value.bio,
            bot: value.bot,
            premium_since: value.premium_since,
        }
    }
}
