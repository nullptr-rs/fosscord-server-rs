CREATE TABLE member_roles (
    _index int NOT NULL,
    role_id varchar(255) NOT NULL,

    INDEX IDX_5d7ddc8a5f9c167f548625e772 (_index),
    INDEX IDX_e9080e7a7997a0170026d5139c (role_id),
    PRIMARY KEY (_index, role_id)
) ENGINE = InnoDB