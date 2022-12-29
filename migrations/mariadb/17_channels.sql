CREATE TABLE channels (
    id varchar(255) NOT NULL,
    created_at datetime NOT NULL,
    name varchar(255) NULL,
    icon text NULL,
    type int NOT NULL,
    last_message_id varchar(255) NULL,
    guild_id varchar(255) NULL,
    parent_id varchar(255) NULL,
    owner_id varchar(255) NULL,
    last_pin_timestamp int NULL,
    default_auto_archive_duration int NULL,
    position int NULL,
    permission_overwrites text NULL,
    video_quality_mode int NULL,
    bitrate int NULL,
    user_limit int NULL,
    nsfw tinyint NULL,
    rate_limit_per_user int NULL,
    topic varchar(255) NULL,
    retention_policy_id varchar(255) NULL,

    flags int NULL,
    default_thread_rate_limit_per_user int NULL,

    PRIMARY KEY (id)
) ENGINE = InnoDB