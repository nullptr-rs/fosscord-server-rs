CREATE TABLE members (
    _index int NOT NULL AUTO_INCREMENT,
    id varchar(255) NOT NULL,
    guild_id varchar(255) NOT NULL,
    nick varchar(255) NULL,
    joined_at datetime NOT NULL,
    premium_since datetime NULL,
    deaf tinyint NOT NULL,
    mute tinyint NOT NULL,
    pending tinyint NOT NULL,
    settings text NOT NULL,
    last_message_id varchar(255) NULL,
    joined_by varchar(255) NULL,

    avatar varchar(255) NULL,
    banner varchar(255) NULL,
    bio varchar(255) NOT NULL,
    communication_disabled_until datetime NULL,

    UNIQUE INDEX IDX_bb2bf9386ac443afbbbf9f12d3 (id, guild_id),
    PRIMARY KEY (_index)
) ENGINE = InnoDB