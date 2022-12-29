CREATE TABLE message_stickers (
    messages_id varchar(255) NOT NULL,
    stickers_id varchar(255) NOT NULL,

    INDEX IDX_40bb6f23e7cc133292e92829d2 (messages_id),
    INDEX IDX_e22a70819d07659c7a71c112a1 (stickers_id),
    PRIMARY KEY (messages_id, stickers_id)
) ENGINE = InnoDB