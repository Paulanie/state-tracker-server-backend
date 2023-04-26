CREATE TABLE Amendments
(
    uid                VARCHAR(256) PRIMARY KEY,
    examenRef          VARCHAR(256),
    triAmendement      NVARCHAR(512),
    texteLegislatifRef VARCHAR(256),
    dateDepot          DATETIME,
    datePublication    DATETIME,
    dateSort           DATETIME DEFAULT NULL,
    etat               NVARCHAR(64),
    sousEtat           NVARCHAR(64),
    representation     NVARCHAR(256),
    article99          BIT
);

CREATE NONCLUSTERED INDEX ncix_amendments_dateDepot ON Amendments (dateDepot);