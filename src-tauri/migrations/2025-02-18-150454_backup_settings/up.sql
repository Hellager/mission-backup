-- Your SQL goes here
CREATE TABLE backup_settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    ignore_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    ignore_patterns VARCHAR NOT NULL DEFAULT 'custom',
    compression_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    compression_format VARCHAR NOT NULL DEFAULT 'zip',
    compression_level INTEGER NOT NULL DEFAULT 6,
    trigger_type VARCHAR NOT NULL DEFAULT 'manual',
    cron_expression VARCHAR,
    backup_limit_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    max_backup_count INTEGER,
    max_backup_size BIGINT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    deleted_at TIMESTAMP
);

CREATE TRIGGER update_backup_settings_timestamp 
    AFTER UPDATE ON backup_settings
BEGIN
    UPDATE backup_settings SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;
