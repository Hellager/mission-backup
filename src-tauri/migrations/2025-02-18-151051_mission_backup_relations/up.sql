-- Your SQL goes here
CREATE TABLE mission_backup_relations (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    mission_id INTEGER NOT NULL,
    backup_setting_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    deleted_at TIMESTAMP,
    
    FOREIGN KEY(mission_id) REFERENCES mission_settings(id),
    FOREIGN KEY(backup_setting_id) REFERENCES backup_settings(id),
    UNIQUE(mission_id, backup_setting_id)
);

CREATE INDEX idx_mission_backup_mission ON mission_backup_relations(mission_id) WHERE is_deleted = FALSE;
CREATE INDEX idx_mission_backup_setting ON mission_backup_relations(backup_setting_id) WHERE is_deleted = FALSE;

CREATE TRIGGER update_mission_backup_relations_timestamp 
    AFTER UPDATE ON mission_backup_relations
BEGIN
    UPDATE mission_backup_relations SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;

CREATE TRIGGER enforce_unique_not_deleted_for_mission_backup_relations
    BEFORE INSERT ON mission_backup_relations
    WHEN NEW.is_deleted = FALSE
BEGIN
    SELECT RAISE(ROLLBACK, 'Duplicate entry found')
    WHERE EXISTS (
        SELECT 1 FROM mission_backup_relations
        WHERE mission_id = NEW.mission_id
        AND backup_setting_id = NEW.backup_setting_id
        AND is_deleted = FALSE
    );
END;