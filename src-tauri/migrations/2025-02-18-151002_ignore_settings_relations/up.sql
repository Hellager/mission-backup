-- Your SQL goes here
CREATE TABLE ignore_settings_relations (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    backup_setting_id INTEGER NOT NULL,
    ignore_keyword_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    deleted_at TIMESTAMP,
    
    FOREIGN KEY(backup_setting_id) REFERENCES backup_settings(id),
    FOREIGN KEY(ignore_keyword_id) REFERENCES ignore_keywords(id),
    UNIQUE(backup_setting_id, ignore_keyword_id)
);

CREATE INDEX idx_ignore_relations_backup ON ignore_settings_relations(backup_setting_id) WHERE is_deleted = FALSE;
CREATE INDEX idx_ignore_relations_keyword ON ignore_settings_relations(ignore_keyword_id) WHERE is_deleted = FALSE;

CREATE TRIGGER update_ignore_relations_timestamp 
    AFTER UPDATE ON ignore_settings_relations
BEGIN
    UPDATE ignore_settings_relations SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;

CREATE TRIGGER enforce_unique_not_deleted_for_ignore_settings_relations
    BEFORE INSERT ON ignore_settings_relations
    WHEN NEW.is_deleted = FALSE
BEGIN
    SELECT RAISE(ROLLBACK, 'Duplicate entry found')
    WHERE EXISTS (
        SELECT 1 FROM ignore_settings_relations
        WHERE backup_setting_id = NEW.backup_setting_id
        AND ignore_keyword_id = NEW.ignore_keyword_id
        AND is_deleted = FALSE
    );
END;
