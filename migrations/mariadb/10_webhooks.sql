CREATE TABLE webhooks (
    id varchar(255) NOT NULL,
    type int NOT NULL,
    name varchar(255) NULL,
    avatar varchar(255) NULL,
    token varchar(255) NULL,
    guild_id varchar(255) NULL,
    channel_id varchar(255) NULL,
    application_id varchar(255) NULL,
    user_id varchar(255) NULL,
    source_guild_id varchar(255) NULL,

    PRIMARY KEY (id)
) ENGINE = InnoDB