CREATE TABLE applications (
    id varchar(255) NOT NULL,
    name varchar(255) NOT NULL,
    icon varchar(255) NULL,
    description varchar(255) NULL,
    bot_public tinyint NOT NULL,
    bot_require_code_grant tinyint NOT NULL,
    terms_of_service_url varchar(255) NULL,
    privacy_policy_url varchar(255) NULL,
    summary varchar(255) NULL,
    verify_key varchar(255) NOT NULL,
    cover_image varchar(255) NULL,
    flags int NOT NULL,
    owner_id varchar(255) NULL,
    team_id varchar(255) NULL,
    _type text NULL,
    hook tinyint NOT NULL,
    redirect_uris text NULL,
    rpc_application_state int NULL,
    store_application_state int NULL,
    verification_state int NULL,
    interactions_endpoint_url varchar(255) NULL,
    integration_public tinyint NULL,
    integration_require_code_grant tinyint NULL,
    discoverability_state int NULL,
    discovery_eligibility_flags int NULL,
    tags text NULL,
    install_params text NULL,
    bot_user_id varchar(255) NULL,

    UNIQUE INDEX REL_2ce5a55796fe4c2f77ece57a64 (bot_user_id),
    PRIMARY KEY (id)
) ENGINE = InnoDB