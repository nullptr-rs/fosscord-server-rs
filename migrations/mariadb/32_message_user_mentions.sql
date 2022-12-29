CREATE TABLE message_user_mentions (
    messages_id varchar(255) NOT NULL,
    users_id varchar(255) NOT NULL,

    INDEX IDX_a343387fc560ef378760681c23 (messages_id),
    INDEX IDX_b831eb18ceebd28976239b1e2f (users_id),
    PRIMARY KEY (messages_id, users_id)
) ENGINE = InnoDB