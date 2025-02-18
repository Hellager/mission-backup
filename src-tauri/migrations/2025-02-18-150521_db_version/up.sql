-- Your SQL goes here
CREATE TABLE db_version (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    version VARCHAR NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO db_version (version) VALUES ('0.0.1');

CREATE TRIGGER update_db_version_timestamp 
    AFTER UPDATE ON db_version
BEGIN
    UPDATE db_version SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;
