-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS update_db_version_timestamp;
DROP TABLE IF EXISTS db_version;
