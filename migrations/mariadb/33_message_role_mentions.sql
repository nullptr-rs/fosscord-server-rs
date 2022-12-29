CREATE TABLE message_role_mentions (
    messages_id varchar(255) NOT NULL,
    roles_id varchar(255) NOT NULL,

    INDEX IDX_a8242cf535337a490b0feaea0b (messages_id),
    INDEX IDX_29d63eb1a458200851bc37d074 (roles_id),
    PRIMARY KEY (messages_id, roles_id)
) ENGINE = InnoDB