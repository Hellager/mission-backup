-- Your SQL goes here
CREATE TABLE db_version (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    version VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    deleted_at TIMESTAMP
);

CREATE TRIGGER update_db_version_timestamp 
    AFTER UPDATE ON db_version
BEGIN
    UPDATE db_version SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;
