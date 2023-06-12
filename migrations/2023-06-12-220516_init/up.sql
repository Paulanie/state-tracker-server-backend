CREATE TABLE professions
(
    id       SERIAL PRIMARY KEY,
    name     VARCHAR(512) UNIQUE NULL,
    family   VARCHAR(512)        NOT NULL,
    category VARCHAR(512)        NOT NULL
);

CREATE INDEX ix_professions_name ON professions (name);
CREATE INDEX ix_professions_family ON professions (family);

CREATE TABLE actors
(
    uid           VARCHAR(32) PRIMARY KEY,
    title         VARCHAR(128) NOT NULL,
    surname       VARCHAR(256) NOT NULL,
    name          VARCHAR(256) NOT NULL,
    alpha         VARCHAR(256) NOT NULL,
    trigram       VARCHAR(8)   NULL,
    birthdate     DATE         NULL,
    birthplace    VARCHAR(512) NULL,
    death_date    DATE         NULL,
    uri_hatvp     VARCHAR(512) NULL,
    profession_id INT REFERENCES professions (id)
);

CREATE TABLE amendments
(
    uid                        VARCHAR(256) PRIMARY KEY,
    examination_ref            VARCHAR(256)                        NOT NULL,
    tri_amendment              VARCHAR(512)                        NOT NULL,
    legislative_text_ref       VARCHAR(256)                        NOT NULL,
    delivery_date              DATE                                NOT NULL,
    publication_date           DATE                                NOT NULL,
    sort_date                  DATE DEFAULT NULL,
    state                      VARCHAR(64)                         NOT NULL,
    sub_state                  VARCHAR(64)                         NOT NULL,
    representation             VARCHAR(256)                        NOT NULL,
    article99                  BOOLEAN                             NOT NULL,
    content_summary            TEXT DEFAULT NULL,
    content_title              TEXT DEFAULT NULL,
    author_type                VARCHAR(256)                        NOT NULL,
    author_uid                 VARCHAR(32) REFERENCES actors (uid) NOT NULL,
    author_political_group_uid VARCHAR(32)                         NOT NULL
);

CREATE INDEX ix_amendments_deliveryDate ON amendments (delivery_date);
CREATE INDEX ix_amendments_state ON amendments (state);

CREATE TABLE organs
(
    uid                    VARCHAR(128) PRIMARY KEY,
    type                   VARCHAR(128) NOT NULL,
    label                  VARCHAR(512) NOT NULL,
    edition_label          VARCHAR(512) NOT NULL,
    short_label            VARCHAR(128) NOT NULL,
    abbreviation_label     VARCHAR(64)  NOT NULL,
    vi_mo_de_start_date    DATE         NULL,
    vi_mo_de_end_date      DATE         NULL,
    vi_mo_de_approval_date DATE         NULL,
    parent_organ_uid       VARCHAR(128) NULL REFERENCES organs (uid),
    chamber                VARCHAR(128) NULL,
    regime                 VARCHAR(128) NOT NULL,
    legislature            INT          NOT NULL,
    number                 INT          NOT NULL,
    region_type            VARCHAR(64)  NULL,
    region_label           VARCHAR(64)  NULL,
    department_code        VARCHAR(4)   NULL,
    department_label       VARCHAR(64)  NULL
);

CREATE INDEX ix_organs_regionLabel ON organs (region_label);
CREATE INDEX ix_organs_departmentCode ON organs (department_code);
CREATE INDEX ix_organs_type ON organs (type);



CREATE TABLE actors_addresses
(
    uid               VARCHAR(32) PRIMARY KEY,
    actor_uid         VARCHAR(32) REFERENCES actors (uid) NOT NULL,
    address_type      INTEGER                             NOT NULL,
    address_type_name VARCHAR(128)                        NOT NULL,
    weight            INTEGER                             NULL,
    affiliate_address VARCHAR(256)                        NULL,
    street_number     VARCHAR(16)                         NOT NULL,
    street_name       VARCHAR(128)                        NOT NULL,
    zip_code          VARCHAR(8)                          NOT NULL,
    city              VARCHAR(128)                        NOT NULL,
    address           VARCHAR(512)                        NULL,
    phone             VARCHAR(32)                         NULL
);

CREATE INDEX ix_actorsAddresses_type ON actors_addresses (address_type);

CREATE TABLE mandates
(
    uid                   VARCHAR(32) PRIMARY KEY,
    actor_uid             VARCHAR(32) REFERENCES actors (uid) NOT NULL,
    term_of_office        VARCHAR(4)                          NULL,
    organ_type            VARCHAR(16)                         NOT NULL,
    start_date            DATE                                NOT NULL,
    publish_date          DATE                                NULL,
    end_date              DATE                                NULL,
    precedence            VARCHAR(4)                          NULL,
    principal_appointment VARCHAR(4)                          NULL,
    quality               VARCHAR(32)                         NOT NULL,
    organ_uid             VARCHAR(32)                         NOT NULL
);

CREATE TABLE parliamentary_mandates
(
    uid                        VARCHAR(32) PRIMARY KEY,
    actor_uid                  VARCHAR(32) REFERENCES actors (uid)                 NOT NULL,
    term_of_office             VARCHAR(4)                                          NULL,
    organ_type                 VARCHAR(16)                                         NOT NULL,
    start_date                 DATE,
    publish_date               DATE                                                NULL,
    end_date                   DATE                                                NULL,
    precedence                 VARCHAR(4)                                          NULL,
    principal_appointment      VARCHAR(4)                                          NULL,
    quality                    VARCHAR(32)                                         NOT NULL,
    organ_uid                  VARCHAR(32)                                         NOT NULL,
    chamber                    VARCHAR(128)                                        NULL,
    election_region            VARCHAR(32)                                         NOT NULL,
    election_region_type       VARCHAR(32)                                         NOT NULL,
    election_department        VARCHAR(32)                                         NOT NULL,
    election_department_number VARCHAR(2)                                          NOT NULL,
    election_district_number   INTEGER                                             NOT NULL,
    election_mandate_cause     VARCHAR(64)                                         NOT NULL,
    election_district          VARCHAR(32)                                         NOT NULL,
    mandate_start              DATE                                                NOT NULL,
    end_reason                 VARCHAR(128)                                        NULL,
    first_election             INTEGER                                             NOT NULL,
    assembly_place             INTEGER                                             NOT NULL,
    replacing_mandate_uid      VARCHAR(32) REFERENCES parliamentary_mandates (uid) NULL
);

CREATE TABLE parliamentary_mandate_collaborators
(
    id          SERIAL PRIMARY KEY,
    mandate_uid VARCHAR(32) REFERENCES parliamentary_mandates (uid) NOT NULL,
    title       VARCHAR(32)                                         NOT NULL,
    surname     VARCHAR(256)                                        NOT NULL,
    name        VARCHAR(256)                                        NOT NULL,
    start_date  DATE                                                NULL,
    end_date    DATE                                                NULL
);