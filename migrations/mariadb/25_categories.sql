CREATE TABLE categories (
    id int NOT NULL,
    name varchar(255) NULL,
    localizations text NOT NULL,
    is_primary tinyint NULL,

    PRIMARY KEY (id)
) ENGINE = InnoDB