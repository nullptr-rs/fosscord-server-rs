CREATE TABLE audit_logs (
    id varchar(255) NOT NULL,
    user_id varchar(255) NULL,
    action_type int NOT NULL,
    options text NULL,
    changes text NOT NULL,
    reason varchar(255) NULL,
    target_id varchar(255) NULL,

    PRIMARY KEY (id)
) ENGINE = InnoDB