CREATE TABLE stickers (
    id varchar(255) NOT NULL,
    name varchar(255) NOT NULL,
    description varchar(255) NULL,
    available tinyint NULL,
    tags varchar(255) NULL,
    pack_id varchar(255) NULL,
    guild_id varchar(255) NULL,
    user_id varchar(255) NULL,
    type int NOT NULL,
    format_type int NOT NULL,

    PRIMARY KEY (id)
) ENGINE = InnoDB