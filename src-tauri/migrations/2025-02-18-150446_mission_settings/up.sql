-- Your SQL goes here
CREATE TABLE mission_settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    description TEXT,
    source_path TEXT NOT NULL,
    target_path TEXT NOT NULL,
    object_type VARCHAR NOT NULL,
    next_run_time TIMESTAMP,
    last_trigger_time TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    deleted_at TIMESTAMP
);

CREATE TRIGGER update_mission_settings_timestamp 
    AFTER UPDATE ON mission_settings
BEGIN
    UPDATE mission_settings SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;
