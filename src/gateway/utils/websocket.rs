use crate::utils::other::intents::Intents;
use crate::utils::other::permission::Permissions;
use crate::utils::other::snowflake::Snowflake;
use actix_web_actors::ws;
use flate2::read::ZlibDecoder;
use std::collections::HashMap;
use tokio::time::Instant;

pub struct WebSocket {
    pub version: i32,
    pub user_id: Snowflake,
    pub session_id: Snowflake,
    pub encoding: String,
    pub compress: Option<String>,
    pub shard_count: Option<i32>,
    pub shard_id: Option<i32>,
    pub deflate: Option<ZlibDecoder<String>>,
    pub heartbeat_timeout: i32,
    pub ready_timeout: i32,
    pub intents: Vec<Intents>,
    pub sequence: i32,
    pub permissions: HashMap<String, Permissions>,
    pub events: HashMap<String, Box<dyn Fn(String)>>,
    pub member_events: HashMap<String, Box<dyn Fn(String)>>,
    pub listen_options: String,

    pub last_heartbeat: Instant,
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub fn set_heartbeat(socket: &mut WebSocket) {}
