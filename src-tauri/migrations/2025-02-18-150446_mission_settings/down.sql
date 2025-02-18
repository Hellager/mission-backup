-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS update_mission_settings_timestamp;
DROP TABLE IF EXISTS mission_settings;
