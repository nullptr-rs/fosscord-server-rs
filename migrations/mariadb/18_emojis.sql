CREATE TABLE emojis (
    id varchar(255) NOT NULL,
    animated tinyint NOT NULL,
    available tinyint NOT NULL,
    guild_id varchar(255) NOT NULL,
    user_id varchar(255) NULL,
    managed tinyint NOT NULL,
    name varchar(255) NOT NULL,
    require_colons tinyint NOT NULL,
    roles text NOT NULL,
    groups text NULL,

    PRIMARY KEY (id)
) ENGINE = InnoDB