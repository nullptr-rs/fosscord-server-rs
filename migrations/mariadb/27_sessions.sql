CREATE TABLE sessions (
    id varchar(255) NOT NULL,
    user_id varchar(255) NULL,
    session_id varchar(255) NOT NULL,
    activities text NULL,
    client_info text NOT NULL,
    status varchar(255) NOT NULL,

    PRIMARY KEY (id)
) ENGINE = InnoDB