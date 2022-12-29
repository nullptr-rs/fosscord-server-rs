CREATE TABLE client_release (
    id varchar(255) NOT NULL,
    name varchar(255) NOT NULL,
    pub_date varchar(255) NOT NULL,
    url varchar(255) NOT NULL,
    deb_url varchar(255) NOT NULL,
    osx_url varchar(255) NOT NULL,
    win_url varchar(255) NOT NULL,
    notes varchar(255) NULL,

    PRIMARY KEY (id)
) ENGINE = InnoDB