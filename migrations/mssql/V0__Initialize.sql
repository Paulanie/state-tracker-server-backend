CREATE TABLE amendments
(
    [uid]                  VARCHAR(256) PRIMARY KEY,
    [examination_ref]      VARCHAR(256),
    [tri_amendment]        NVARCHAR(512),
    [legislative_text_ref] VARCHAR(256),
    [delivery_date]        DATETIME,
    [publication_date]     DATETIME,
    [sort_date]            DATETIME DEFAULT NULL,
    [state]                NVARCHAR(64),
    [sub_state]            NVARCHAR(64),
    [representation]       NVARCHAR(256),
    [article99]            BIT
);

CREATE NONCLUSTERED INDEX ncix_amendments_deliveryDate ON amendments (delivery_date);
CREATE NONCLUSTERED INDEX ncix_amendments_state ON amendments (state);

CREATE TABLE organs
(
    [uid]                    VARCHAR(128) PRIMARY KEY,
    [type]                   NVARCHAR(128),
    [label]                  NVARCHAR(512),
    [edition_label]          NVARCHAR(512),
    [short_label]            NVARCHAR(128),
    [abbreviation_label]     NVARCHAR(64),
    [vi_mo_de_start_date]    DATE          NULL,
    [vi_mo_de_end_date]      DATE          NULL,
    [vi_mo_de_approval_date] DATE          NULL,
    [parent_organ_uid]       VARCHAR(128)  NULL FOREIGN KEY REFERENCES organs (uid),
    [chamber]                NVARCHAR(128) NULL,
    [regime]                 NVARCHAR(128),
    [legislature]            INT,
    [number]                 INT,
    [region_type]            NVARCHAR(64)  NULL,
    [region_label]           NVARCHAR(64)  NULL,
    [department_code]        VARCHAR(4)    NULL,
    [department_label]       NVARCHAR(64)  NULL
);

CREATE NONCLUSTERED INDEX ncix_organs_regionLabel ON organs (region_label);
CREATE NONCLUSTERED INDEX ncix_organs_departmentCode ON organs (department_code);
CREATE NONCLUSTERED INDEX ncix_organs_type ON organs (type);


CREATE TABLE professions
(
    [id]       INT IDENTITY (1,1) PRIMARY KEY,
    [name]     NVARCHAR(512) UNIQUE NULL,
    [family]   NVARCHAR(512),
    [category] NVARCHAR(512)
);

CREATE NONCLUSTERED INDEX ncix_professions_name ON professions (name);
CREATE NONCLUSTERED INDEX ncix_professions_family ON professions (family);

CREATE TABLE actors
(
    [uid]           VARCHAR(32) PRIMARY KEY,
    [title]         NVARCHAR(128),
    [surname]       NVARCHAR(256),
    [name]          NVARCHAR(256),
    [alpha]         NVARCHAR(256),
    [trigram]       VARCHAR(8)    NULL,
    [birthdate]     DATE          NULL,
    [birthplace]    NVARCHAR(512) NULL,
    [death_date]    DATE          NULL,
    [uri_hatvp]     NVARCHAR(512) NULL,
    [profession_id] INT FOREIGN KEY REFERENCES professions (id),
);

CREATE TABLE actors_addresses
(
    [uid]               VARCHAR(32) PRIMARY KEY,
    [actor_uid]         VARCHAR(32) FOREIGN KEY REFERENCES actors (uid),
    [type]              INTEGER,
    [type_name]         NVARCHAR(128),
    [weight]            INTEGER      NULL,
    [affiliate_address] VARCHAR(256) NULL,
    [street_number]     VARCHAR(16),
    [street_name]       VARCHAR(128),
    [zip_code]          VARCHAR(8),
    [city]              VARCHAR(128),
    [address]           VARCHAR(512) NULL,
    [phone]             VARCHAR(32)  NULL
);

CREATE NONCLUSTERED INDEX ncix_actorsAddresses_type ON actors_addresses (type);

CREATE TABLE mandates
(
    [uid]                   VARCHAR(32) PRIMARY KEY,
    [actor_uid]             VARCHAR(32) FOREIGN KEY REFERENCES actors (uid),
    [term_of_office]        VARCHAR(4) NULL,
    [organ_type]            VARCHAR(16),
    [start_date]            DATE,
    [publish_date]          DATE       NULL,
    [end_date]              DATE       NULL,
    [precedence]            VARCHAR(4) NULL,
    [principal_appointment] VARCHAR(4) NULL,
    [quality]               VARCHAR(32),
    [organ_uid]             VARCHAR(32)
);

CREATE TABLE parliamentary_mandates
(
    [uid]                        VARCHAR(32) PRIMARY KEY,
    [actor_uid]                  VARCHAR(32) FOREIGN KEY REFERENCES actors (uid),
    [term_of_office]             VARCHAR(4)                                                      NULL,
    [organ_type]                 VARCHAR(16),
    [start_date]                 DATE,
    [publish_date]               DATE                                                            NULL,
    [end_date]                   DATE                                                            NULL,
    [precedence]                 VARCHAR(4)                                                      NULL,
    [principal_appointment]      VARCHAR(4)                                                      NULL,
    [quality]                    NVARCHAR(32),
    [organ_uid]                  VARCHAR(32),
    [chamber]                    VARCHAR(128)                                                    NULL,
    [election_region]            NVARCHAR(32),
    [election_region_type]       NVARCHAR(32),
    [election_department]        NVARCHAR(32),
    [election_department_number] VARCHAR(2),
    [election_district_number]   INTEGER,
    [election_mandate_cause]     NVARCHAR(64),
    [election_district]          VARCHAR(32),
    [mandate_start]              DATE,
    [end_reason]                 VARCHAR(128)                                                    NULL,
    [first_election]             INTEGER,
    [assembly_place]             INTEGER,
    [replacing_mandate_uid]      VARCHAR(32) FOREIGN KEY REFERENCES parliamentary_mandates (uid) NULL
);

CREATE TABLE parliamentary_mandate_collaborators
(
    [id]          INT IDENTITY (1,1) PRIMARY KEY,
    [mandate_uid] VARCHAR(32) FOREIGN KEY REFERENCES parliamentary_mandates (uid),
    [title]       NVARCHAR(32),
    [surname]     NVARCHAR(256),
    [name]        NVARCHAR(256),
    [start_date]  DATE NULL,
    [end_date]    DATE NULL
);