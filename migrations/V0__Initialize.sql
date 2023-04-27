CREATE TABLE Amendments
(
    [uid]                VARCHAR(256) PRIMARY KEY,
    [examenRef]          VARCHAR(256),
    [triAmendement]      NVARCHAR(512),
    [texteLegislatifRef] VARCHAR(256),
    [dateDepot]          DATETIME,
    [datePublication]    DATETIME,
    [dateSort]           DATETIME DEFAULT NULL,
    [etat]               NVARCHAR(64),
    [sousEtat]           NVARCHAR(64),
    [representation]     NVARCHAR(256),
    [article99]          BIT
);

CREATE NONCLUSTERED INDEX ncix_amendments_dateDepot ON Amendments (dateDepot);
CREATE NONCLUSTERED INDEX ncix_amendments_etat ON Amendments (etat);

CREATE TABLE Actors
(
    [uid]          VARCHAR(32) PRIMARY KEY,
    [title]        NVARCHAR(128),
    [surname]      NVARCHAR(256),
    [name]         NVARCHAR(256),
    [alpha]        NVARCHAR(256),
    [trigram]      CHAR(3),
    [birthdate]    DATE,
    [birthplace]   NVARCHAR(512),
    [deathDate]    DATE          NULL,
    [uriHatvp]     NVARCHAR(512) NULL,
    [professionId] INT FOREIGN KEY REFERENCES Professions (id),
);

CREATE TABLE Professions
(
    [id]       INT IDENTITY (1,1) PRIMARY KEY,
    [name]     NVARCHAR(512) UNIQUE,
    [family]   NVARCHAR(512),
    [category] NVARCHAR(512)
);

CREATE NONCLUSTERED INDEX ncix_professions_name ON Professions (name);


CREATE TABLE ActorPostalAddresses
(
    [uid]              VARCHAR(32) PRIMARY KEY,
    [actorUid]         VARCHAR(32) FOREIGN KEY REFERENCES Actors (uid),
    [type]             INTEGER,
    [typeName]         NVARCHAR(128),
    [weight]           INTEGER      NULL,
    [affiliateAddress] VARCHAR(256) NULL,
    [streetNumber]     VARCHAR(8),
    [streetName]       VARCHAR(128),
    [zipCode]          VARCHAR(8),
    [city]             VARCHAR(128)
);

CREATE TABLE ActorWebsites
(
    [uid]              VARCHAR(32) PRIMARY KEY,
    [actorUid]         VARCHAR(32) FOREIGN KEY REFERENCES Actors (uid),
    [type]             INTEGER,
    [typeName]         NVARCHAR(128),
    [weight]           INTEGER      NULL,
    [affiliateAddress] VARCHAR(256) NULL,
    [address]          VARCHAR(512)
);

CREATE TABLE ActorMails
(
    [uid]              VARCHAR(32) PRIMARY KEY,
    [actorUid]         VARCHAR(32) FOREIGN KEY REFERENCES Actors (uid),
    [type]             INTEGER,
    [typeName]         NVARCHAR(128),
    [weight]           INTEGER      NULL,
    [affiliateAddress] VARCHAR(256) NULL,
    [address]          VARCHAR(512)
);

CREATE TABLE ActorPhones
(
    [uid]              VARCHAR(32) PRIMARY KEY,
    [actorUid]         VARCHAR(32) FOREIGN KEY REFERENCES Actors (uid),
    [type]             INTEGER,
    [typeName]         NVARCHAR(128),
    [weight]           INTEGER      NULL,
    [affiliateAddress] VARCHAR(256) NULL,
    [phone]            VARCHAR(16)
);

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
)

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
)

CREATE TABLE ParliamentaryMandateCollaborators
(
    [id]         INT IDENTITY (1,1) PRIMARY KEY,
    [mandateUid] VARCHAR(32) FOREIGN KEY REFERENCES ParliamentaryMandates (uid),
    [title]      NVARCHAR(32),
    [surname]    NVARCHAR(256),
    [name]       NVARCHAR(256),
    [startDate]  DATE NULL,
    [endDate]    DATE NULL
)