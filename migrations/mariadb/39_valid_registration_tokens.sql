CREATE TABLE valid_registration_tokens (
    token varchar(255) NOT NULL,
    created_at datetime NOT NULL,
    expires_at datetime NOT NULL,

    PRIMARY KEY (token)
) ENGINE = InnoDB