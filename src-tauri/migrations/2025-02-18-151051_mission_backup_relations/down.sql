-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS update_mission_backup_relations_timestamp;
DROP INDEX IF EXISTS idx_mission_backup_mission;
DROP INDEX IF EXISTS idx_mission_backup_setting;
DROP TABLE IF EXISTS mission_backup_relations;
