CREATE TABLE templates (
    id varchar(255) NOT NULL,
    code varchar(255) NOT NULL,
    name varchar(255) NOT NULL,
    description varchar(255) NULL,
    usage_count int NULL,
    creator_id varchar(255) NULL,
    created_at datetime NOT NULL,
    updated_at datetime NOT NULL,
    source_guild_id varchar(255) NULL,
    serialized_source_guild text NOT NULL,

    UNIQUE INDEX IDX_be38737bf339baf63b1daeffb5 (code),
    PRIMARY KEY (id)
) ENGINE = InnoDB