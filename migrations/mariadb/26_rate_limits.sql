CREATE TABLE rate_limits (
    id varchar(255) NOT NULL,
    executor_id varchar(255) NOT NULL,
    hits int NOT NULL,
    blocked tinyint NOT NULL,
    expires_at datetime NOT NULL,

    PRIMARY KEY (id)
) ENGINE = InnoDB