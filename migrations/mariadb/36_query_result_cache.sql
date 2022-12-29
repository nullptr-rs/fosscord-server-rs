CREATE TABLE query_result_cache (
    id int NOT NULL AUTO_INCREMENT,
    identifier varchar(255) NULL,
    time bigint NOT NULL,
    duration int NOT NULL,
    query text NOT NULL,
    result text NOT NULL,

    PRIMARY KEY (id)
) ENGINE = InnoDB