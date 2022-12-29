CREATE TABLE invites (
    code varchar(255) NOT NULL,
    temporary tinyint NOT NULL,
    uses int NOT NULL,
    max_uses int NOT NULL,
    max_age int NOT NULL,
    created_at datetime NOT NULL,
    expires_at datetime NOT NULL,
    guild_id varchar(255) NULL,
    channel_id varchar(255) NULL,
    inviter_id varchar(255) NULL,
    target_user_id varchar(255) NULL,
    target_user_type int NULL,
    vanity_url tinyint NULL,

    PRIMARY KEY (code)
) ENGINE = InnoDB