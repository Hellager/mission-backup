-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS update_backup_settings_timestamp;
DROP TABLE IF EXISTS backup_settings;
