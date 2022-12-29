CREATE TABLE message_channel_mentions (
    messages_id varchar(255) NOT NULL,
    channels_id varchar(255) NOT NULL,

    INDEX IDX_2a27102ecd1d81b4582a436092 (messages_id),
    INDEX IDX_bdb8c09e1464cabf62105bf4b9 (channels_id),
    PRIMARY KEY (messages_id, channels_id)
) ENGINE = InnoDB