CREATE TABLE team_members (
    id varchar(255) NOT NULL,
    membership_state int NOT NULL,
    permissions text NOT NULL,
    team_id varchar(255) NULL,
    user_id varchar(255) NULL,

    PRIMARY KEY (id)
) ENGINE = InnoDB