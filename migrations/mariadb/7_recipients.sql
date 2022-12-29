CREATE TABLE recipients (
    id varchar(255) NOT NULL,
    channel_id varchar(255) NOT NULL,
    user_id varchar(255) NOT NULL,
    closed tinyint NOT NULL DEFAULT 0,

    PRIMARY KEY (id)
) ENGINE = InnoDB