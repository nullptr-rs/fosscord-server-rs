CREATE TABLE attachments (
    id varchar(255) NOT NULL,
    filename varchar(255) NOT NULL,
    size int NOT NULL,
    url varchar(255) NOT NULL,
    proxy_url varchar(255) NOT NULL,
    height int NULL,
    width int NULL,
    content_type varchar(255) NULL,
    message_id varchar(255) NULL,

    PRIMARY KEY (id)
) ENGINE = InnoDB