CREATE TABLE read_states (
    id varchar(255) NOT NULL,
    channel_id varchar(255) NOT NULL,
    user_id varchar(255) NOT NULL,
    last_message_id varchar(255) NULL,
    public_ack varchar(255) NULL,
    notifications_cursor varchar(255) NULL,
    last_pin_timestamp datetime NULL,
    mention_count int NULL,

    UNIQUE INDEX IDX_0abf8b443321bd3cf7f81ee17a (channel_id, user_id),
    PRIMARY KEY (id)
) ENGINE = InnoDB