-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS update_ignore_relations_timestamp;
DROP INDEX IF EXISTS idx_ignore_relations_backup;
DROP INDEX IF EXISTS idx_ignore_relations_keyword;
DROP TABLE IF EXISTS ignore_settings_relations;
