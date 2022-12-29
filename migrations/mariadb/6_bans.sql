CREATE TABLE bans (
    id varchar(255) NOT NULL,
    user_id varchar(255) NULL,
    guild_id varchar(255) NULL,
    executor_id varchar(255) NULL,
    ip varchar(255) NOT NULL,
    reason varchar(255) NULL,
    
    PRIMARY KEY (id)
) ENGINE = InnoDB