-- Your SQL goes here
CREATE TABLE backup_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    mission_id INTEGER NOT NULL, 
    backup_path TEXT NOT NULL,
    file_size BIGINT NOT NULL,
    file_hash TEXT,
    status TEXT NOT NULL DEFAULT 'completed',
    error_message TEXT,
    compression_format TEXT,
    compression_level INTEGER,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    deleted_at TIMESTAMP,
    
    FOREIGN KEY(mission_id) REFERENCES mission_settings(id)
);

CREATE INDEX idx_backup_records_task ON backup_records(mission_id) WHERE is_deleted = FALSE;
CREATE INDEX idx_backup_records_status ON backup_records(status) WHERE is_deleted = FALSE;

CREATE TRIGGER update_backup_records_timestamp 
    AFTER UPDATE ON backup_records
BEGIN
    UPDATE backup_records SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;
