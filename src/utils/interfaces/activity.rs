use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::utils::other::snowflake::Snowflake;

#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub name: String,
    pub _type: Type,
    pub url: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub timestamps: Option<Timestamps>,
    pub application_id: Option<String>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<Emoji>,
    pub party: Option<Party>,
    pub assets: Option<Assets>,
    pub secrets: Option<Secrets>,
    pub instance: bool,
    pub flags: String,

    pub id: Option<String>,
    pub sync_id: Option<String>,
    pub metadata: Option<Metadata>,
    pub session_id: Option<String>
}

pub enum Type {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Custom = 4,
    Competing = 5
}

pub struct Timestamps {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime
}

pub struct Emoji {
    pub name: String,
    pub id: Option<String>,
    pub animated: bool
}

pub struct Party {
    pub id: String,
    pub size: [i32; 2]
}

pub struct Assets {
    pub large_image: Snowflake,
    pub large_text: String,
    pub small_image: Snowflake,
    pub small_text: String
}

pub struct Secrets {
    pub join: String,
    pub spectate: String,
    pub _match: String
}

pub struct Metadata {
    pub context_uri: Option<String>,
    pub album_id: Option<String>,
    pub artist_ids: Option<Vec<String>>,
}