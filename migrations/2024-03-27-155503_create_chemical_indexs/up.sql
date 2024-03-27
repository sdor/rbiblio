-- Your SQL goes here

CREATE INDEX chemicals_pmid_idx ON chemicals (pmid);
CREATE INDEX chemicals_name_of_substance_idx ON chemicals (name_of_substance);
