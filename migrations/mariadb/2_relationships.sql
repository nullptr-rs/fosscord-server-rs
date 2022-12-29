CREATE TABLE relationships (
    id varchar(255) NOT NULL,
    from_id varchar(255) NOT NULL,
    to_id varchar(255) NOT NULL,
    nickname varchar(255) NULL,
    type int NOT NULL,

    UNIQUE INDEX IDX_a0b2ff0a598df0b0d055934a17 (from_id, to_id),
    PRIMARY KEY (id)
) ENGINE = InnoDB