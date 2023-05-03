CREATE TABLE Amendments
(
    [uid]                VARCHAR(256) PRIMARY KEY,
    [examinationRef]     VARCHAR(256),
    [triAmendment]       NVARCHAR(512),
    [legislativeTextRef] VARCHAR(256),
    [deliveryDate]       DATETIME,
    [publicationDate]    DATETIME,
    [sortDate]           DATETIME DEFAULT NULL,
    [state]              NVARCHAR(64),
    [subState]           NVARCHAR(64),
    [representation]     NVARCHAR(256),
    [article99]          BIT
);

CREATE NONCLUSTERED INDEX ncix_amendments_deliveryDate ON Amendments (deliveryDate);
CREATE NONCLUSTERED INDEX ncix_amendments_state ON Amendments (state);

CREATE TABLE Organs
(
    [uid]                VARCHAR(128) PRIMARY KEY,
    [type]               NVARCHAR(128),
    [label]              NVARCHAR(512),
    [editionLabel]       NVARCHAR(512),
    [shortLabel]         NVARCHAR(128),
    [abbreviationLabel]  NVARCHAR(64),
    [viMoDeStartDate]    DATE          NULL,
    [viMoDeEndDate]      DATE          NULL,
    [viMoDeApprovalDate] DATE          NULL,
    [parentOrganUid]     VARCHAR(128)  NULL FOREIGN KEY REFERENCES Organs (uid),
    [chamber]            NVARCHAR(128) NULL,
    [regime]             NVARCHAR(128),
    [legislature]        INT,
    [number]             INT,
    [regionType]         NVARCHAR(64)  NULL,
    [regionLabel]        NVARCHAR(64)  NULL,
    [departmentCode]     VARCHAR(4)    NULL,
    [departmentLabel]    NVARCHAR(64)  NULL
);

CREATE NONCLUSTERED INDEX ncix_organs_regionLabel ON Organs (regionLabel);
CREATE NONCLUSTERED INDEX ncix_organs_departmentCode ON Organs (departmentCode);
CREATE NONCLUSTERED INDEX ncix_organs_type ON Organs (type);


CREATE TABLE Professions
(
    [id]       INT IDENTITY (1,1) PRIMARY KEY,
    [name]     NVARCHAR(512) UNIQUE NULL,
    [family]   NVARCHAR(512),
    [category] NVARCHAR(512)
);

CREATE NONCLUSTERED INDEX ncix_professions_name ON Professions (name);
CREATE NONCLUSTERED INDEX ncix_professions_family ON Professions (family);

CREATE TABLE Actors
(
    [uid]          VARCHAR(32) PRIMARY KEY,
    [title]        NVARCHAR(128),
    [surname]      NVARCHAR(256),
    [name]         NVARCHAR(256),
    [alpha]        NVARCHAR(256),
    [trigram]      VARCHAR(8)    NULL,
    [birthdate]    DATE          NULL,
    [birthplace]   NVARCHAR(512) NULL,
    [deathDate]    DATE          NULL,
    [uriHatvp]     NVARCHAR(512) NULL,
    [professionId] INT FOREIGN KEY REFERENCES Professions (id),
);

CREATE TABLE ActorsAddresses
(
    [uid]              VARCHAR(32) PRIMARY KEY,
    [actorUid]         VARCHAR(32) FOREIGN KEY REFERENCES Actors (uid),
    [type]             INTEGER,
    [typeName]         NVARCHAR(128),
    [weight]           INTEGER      NULL,
    [affiliateAddress] VARCHAR(256) NULL,
    [streetNumber]     VARCHAR(16),
    [streetName]       VARCHAR(128),
    [zipCode]          VARCHAR(8),
    [city]             VARCHAR(128),
    [address]          VARCHAR(512) NULL,
    [phone]            VARCHAR(32)  NULL
);

CREATE NONCLUSTERED INDEX ncix_actorsAddresses_type ON ActorsAddresses (type);

CREATE TABLE Mandates
(
    [uid]                  VARCHAR(32) PRIMARY KEY,
    [actorUid]             VARCHAR(32) FOREIGN KEY REFERENCES Actors (uid),
    [termOfOffice]         VARCHAR(4) NULL,
    [organType]            VARCHAR(16),
    [startDate]            DATE,
    [publishDate]          DATE       NULL,
    [endDate]              DATE       NULL,
    [precedence]           VARCHAR(4) NULL,
    [principalAppointment] VARCHAR(4) NULL,
    [quality]              VARCHAR(32),
    [organUid]             VARCHAR(32)
);

CREATE TABLE ParliamentaryMandates
(
    [uid]                      VARCHAR(32) PRIMARY KEY,
    [actorUid]                 VARCHAR(32) FOREIGN KEY REFERENCES Actors (uid),
    [termOfOffice]             VARCHAR(4)                                                     NULL,
    [organType]                VARCHAR(16),
    [startDate]                DATE,
    [publishDate]              DATE                                                           NULL,
    [endDate]                  DATE                                                           NULL,
    [precedence]               VARCHAR(4)                                                     NULL,
    [principalAppointment]     VARCHAR(4)                                                     NULL,
    [quality]                  NVARCHAR(32),
    [organUid]                 VARCHAR(32),
    [chamber]                  VARCHAR(128)                                                   NULL,
    [electionRegion]           NVARCHAR(32),
    [electionRegionType]       NVARCHAR(32),
    [electionDepartment]       NVARCHAR(32),
    [electionDepartmentNumber] VARCHAR(2),
    [electionDistrictNumber]   INTEGER,
    [electionMandateCause]     NVARCHAR(64),
    [electionDistrict]         VARCHAR(32),
    [mandateStart]             DATE,
    [endReason]                VARCHAR(128)                                                   NULL,
    [firstElection]            INTEGER,
    [assemblyPlace]            INTEGER,
    [replacingMandateUid]      VARCHAR(32) FOREIGN KEY REFERENCES ParliamentaryMandates (uid) NULL
);

CREATE TABLE ParliamentaryMandateCollaborators
(
    [id]         INT IDENTITY (1,1) PRIMARY KEY,
    [mandateUid] VARCHAR(32) FOREIGN KEY REFERENCES ParliamentaryMandates (uid),
    [title]      NVARCHAR(32),
    [surname]    NVARCHAR(256),
    [name]       NVARCHAR(256),
    [startDate]  DATE NULL,
    [endDate]    DATE NULL
);