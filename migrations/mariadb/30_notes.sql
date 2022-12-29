CREATE TABLE notes (
    id varchar(255) NOT NULL,
    content varchar(255) NOT NULL,
    owner_id varchar(255) NULL,
    target_id varchar(255) NULL,

    UNIQUE INDEX IDX_74e6689b9568cc965b8bfc9150 (owner_id, target_id),
    PRIMARY KEY (id)
) ENGINE = InnoDB