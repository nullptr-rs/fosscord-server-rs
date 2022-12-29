CREATE TABLE connected_accounts (
    id varchar(255) NOT NULL,
    user_id varchar(255) NULL,
    access_token varchar(255) NOT NULL,
    friend_sync tinyint NOT NULL,
    name varchar(255) NOT NULL,
    revoked tinyint NOT NULL,
    show_activity tinyint NOT NULL,
    type varchar(255) NOT NULL,
    verified tinyint NOT NULL,
    visibility int NOT NULL,

    PRIMARY KEY (id)
) Engine = InnoDB