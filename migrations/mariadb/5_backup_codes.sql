CREATE TABLE backup_codes (
    id varchar(255) NOT NULL,
    code varchar(255) NOT NULL,
    consumed tinyint NOT NULL,
    expired tinyint NOT NULL,
    user_id varchar(255) NULL,

    PRIMARY KEY (id)
) ENGINE = InnoDB