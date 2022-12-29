CREATE TABLE sticker_packs (
    id varchar(255) NOT NULL,
    name varchar(255) NOT NULL,
    description varchar(255) NULL,
    banner_asset_id varchar(255) NULL,
    cover_sticker_id varchar(255) NULL,
    coverStickerId varchar(255) NULL,

    PRIMARY KEY (id)
) ENGINE = InnoDB