CREATE TABLE roles (
    id varchar(255) NOT NULL,
    guild_id varchar(255) NULL,
    color int NOT NULL,
    hoist tinyint NOT NULL,
    managed tinyint NOT NULL,
    mentionable tinyint NOT NULL,
    name varchar(255) NOT NULL,
    permissions varchar(255) NOT NULL,
    position int NOT NULL,
    icon varchar(255) NULL,
    unicode_emoji varchar(255) NULL,
    tags text NULL,

    PRIMARY KEY (id)
) ENGINE = InnoDB