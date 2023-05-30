CREATE TABLE amendments
(
    uid                        VARCHAR(256) PRIMARY KEY,
    examination_ref            VARCHAR(256),
    tri_amendment              VARCHAR(512),
    legislative_text_ref       VARCHAR(256),
    delivery_date              DATE,
    publication_date           DATE,
    sort_date                  DATE DEFAULT NULL,
    state                      VARCHAR(64),
    sub_state                  VARCHAR(64),
    representation             VARCHAR(256),
    article99                  BOOLEAN,
    content_summary            TEXT,
    content_title              TEXT,
    author_type                VARCHAR(256),
    author_uid                 VARCHAR(32) REFERENCES actors (uid),
    author_political_group_uid VARCHAR(32)
);

CREATE INDEX ix_amendments_deliveryDate ON amendments (delivery_date);
CREATE INDEX ix_amendments_state ON amendments (state);

CREATE TABLE organs
(
    uid                    VARCHAR(128) PRIMARY KEY,
    type                   VARCHAR(128),
    label                  VARCHAR(512),
    edition_label          VARCHAR(512),
    short_label            VARCHAR(128),
    abbreviation_label     VARCHAR(64),
    vi_mo_de_start_date    DATE         NULL,
    vi_mo_de_end_date      DATE         NULL,
    vi_mo_de_approval_date DATE         NULL,
    parent_organ_uid       VARCHAR(128) NULL REFERENCES organs (uid),
    chamber                VARCHAR(128) NULL,
    regime                 VARCHAR(128),
    legislature            INT,
    number                 INT,
    region_type            VARCHAR(64)  NULL,
    region_label           VARCHAR(64)  NULL,
    department_code        VARCHAR(4)   NULL,
    department_label       VARCHAR(64)  NULL
);

CREATE INDEX ix_organs_regionLabel ON organs (region_label);
CREATE INDEX ix_organs_departmentCode ON organs (department_code);
CREATE INDEX ix_organs_type ON organs (type);

CREATE TABLE professions
(
    id       SERIAL PRIMARY KEY,
    name     VARCHAR(512) UNIQUE NULL,
    family   VARCHAR(512),
    category VARCHAR(512)
);

CREATE INDEX ix_professions_name ON professions (name);
CREATE INDEX ix_professions_family ON professions (family);

CREATE TABLE actors
(
    uid           VARCHAR(32) PRIMARY KEY,
    title         VARCHAR(128),
    surname       VARCHAR(256),
    name          VARCHAR(256),
    alpha         VARCHAR(256),
    trigram       VARCHAR(8)   NULL,
    birthdate     DATE         NULL,
    birthplace    VARCHAR(512) NULL,
    death_date    DATE         NULL,
    uri_hatvp     VARCHAR(512) NULL,
    profession_id INT REFERENCES professions (id)
);

CREATE TABLE actors_addresses
(
    uid               VARCHAR(32) PRIMARY KEY,
    actor_uid         VARCHAR(32) REFERENCES actors (uid),
    type              INTEGER,
    type_name         VARCHAR(128),
    weight            INTEGER      NULL,
    affiliate_address VARCHAR(256) NULL,
    street_number     VARCHAR(16),
    street_name       VARCHAR(128),
    zip_code          VARCHAR(8),
    city              VARCHAR(128),
    address           VARCHAR(512) NULL,
    phone             VARCHAR(32)  NULL
);

CREATE INDEX ix_actorsAddresses_type ON actors_addresses (type);

CREATE TABLE mandates
(
    uid                   VARCHAR(32) PRIMARY KEY,
    actor_uid             VARCHAR(32) REFERENCES actors (uid),
    term_of_office        VARCHAR(4) NULL,
    organ_type            VARCHAR(16),
    start_date            DATE,
    publish_date          DATE       NULL,
    end_date              DATE       NULL,
    precedence            VARCHAR(4) NULL,
    principal_appointment VARCHAR(4) NULL,
    quality               VARCHAR(32),
    organ_uid             VARCHAR(32)
);

CREATE TABLE parliamentary_mandates
(
    uid                        VARCHAR(32) PRIMARY KEY,
    actor_uid                  VARCHAR(32) REFERENCES actors (uid),
    term_of_office             VARCHAR(4)                                          NULL,
    organ_type                 VARCHAR(16),
    start_date                 DATE,
    publish_date               DATE                                                NULL,
    end_date                   DATE                                                NULL,
    precedence                 VARCHAR(4)                                          NULL,
    principal_appointment      VARCHAR(4)                                          NULL,
    quality                    VARCHAR(32),
    organ_uid                  VARCHAR(32),
    chamber                    VARCHAR(128)                                        NULL,
    election_region            VARCHAR(32),
    election_region_type       VARCHAR(32),
    election_department        VARCHAR(32),
    election_department_number VARCHAR(2),
    election_district_number   INTEGER,
    election_mandate_cause     VARCHAR(64),
    election_district          VARCHAR(32),
    mandate_start              DATE,
    end_reason                 VARCHAR(128)                                        NULL,
    first_election             INTEGER,
    assembly_place             INTEGER,
    replacing_mandate_uid      VARCHAR(32) REFERENCES parliamentary_mandates (uid) NULL
);

CREATE TABLE parliamentary_mandate_collaborators
(
    id          SERIAL PRIMARY KEY,
    mandate_uid VARCHAR(32) REFERENCES parliamentary_mandates (uid),
    title       VARCHAR(32),
    surname     VARCHAR(256),
    name        VARCHAR(256),
    start_date  DATE NULL,
    end_date    DATE NULL
);