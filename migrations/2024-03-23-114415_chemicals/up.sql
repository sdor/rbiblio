-- Your SQL goes here
CREATE TABLE chemicals (
    id SERIAL PRIMARY KEY,
    pmid INT NOT NULL,
    registry_number VARCHAR(255) NOT NULL,
    name_of_substance VARCHAR(255) NOT NULL,
    year INT NOT NULL,
    name_of_substance_tsv tsvector
);
CREATE INDEX chemicals_year_idx ON chemicals (year);
CREATE INDEX chemicals_name_of_substance_tsv_gin ON chemicals USING gin (name_of_substance_tsv);
CREATE TRIGGER chemicals_name_of_substance_tsv_trigger BEFORE INSERT OR UPDATE ON chemicals FOR EACH ROW EXECUTE PROCEDURE tsvector_update_trigger(name_of_substance_tsv, 'pg_catalog.english', name_of_substance);
