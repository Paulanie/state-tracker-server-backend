DROP TABLE amendments;
DROP INDEX ix_amendments_deliveryDate;
DROP INDEX ix_amendments_state;

DROP TABLE organs;

DROP INDEX ix_organs_regionLabel;
DROP INDEX ix_organs_departmentCode;
DROP INDEX ix_organs_type;

DROP TABLE professions;

DROP INDEX ix_professions_name;
DROP INDEX ix_professions_family;

DROP TABLE actors;
DROP TABLE actors_addresses;
DROP INDEX ix_actorsAddresses_type;
DROP TABLE mandates;
DROP TABLE parliamentary_mandates;
DROP TABLE parliamentary_mandate_collaborators;