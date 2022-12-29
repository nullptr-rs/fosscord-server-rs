CREATE TABLE teams (
    id varchar(255) NOT NULL,
    icon varchar(255) NULL,
    name varchar(255) NOT NULL,
    owner_user_id varchar(255) NULL,

    PRIMARY KEY (id)
) ENGINE = InnoDB