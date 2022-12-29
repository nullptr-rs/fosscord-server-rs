CREATE TABLE voice_states (
    id varchar(255) NOT NULL,
    guild_id varchar(255) NULL,
    channel_id varchar(255) NULL,
    user_id varchar(255) NULL,
    session_id varchar(255) NOT NULL,
    token varchar(255) NULL,
    deaf tinyint NOT NULL,
    mute tinyint NOT NULL,
    self_deaf tinyint NOT NULL,
    self_mute tinyint NOT NULL,
    self_stream tinyint NULL,
    self_video tinyint NOT NULL,
    suppress tinyint NOT NULL,
    request_to_speak_timestamp datetime NULL,

    PRIMARY KEY (id)
) ENGINE = InnoDB