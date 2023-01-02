use serde_json::Value;

enum OpCodes {
    Dispatch = 0,
    Heartbeat = 1,
    Identify = 2,
    PresenceUpdate = 3,
    VoiceStateUpdate = 4,
    VoiceServerPing = 5, // ? What is opcode 5?
    Resume = 6,
    Reconnect = 7,
    RequestGuildMembers = 8,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatAck = 11,
    GuildSync = 12,
    DmUpdate = 13,
    LazyRequest = 14,
    LobbyConnect = 15,
    LobbyDisconnect = 16,
    LobbyVoiceStatesUpdate = 17,
    StreamCreate = 18,
    StreamDelete = 19,
    StreamWatch = 20,
    StreamPing = 21,
    StreamSetPaused = 22,
    RequestApplicationCommands = 24
}

pub enum CloseCodes {
    UnknownError = 4000,
    UnknownOpcode,
    DecodeError,
    NotAuthenticated,
    AuthenticationFailed,
    AlreadyAuthenticated,
    InvalidSession,
    InvalidSeq,
    RateLimited,
    SessionTimedOut,
    InvalidShard,
    ShardingRequired,
    InvalidApiVersion,
    InvalidIntent,
    DisallowedIntent
}

pub struct Payload {
    op: OpCodes,
    d: Option<Value>,
    s: Option<i32>,
    t: Option<String>
}