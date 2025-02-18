-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS update_backup_records_timestamp;
DROP INDEX IF EXISTS idx_backup_records_task;
DROP INDEX IF EXISTS idx_backup_records_status;
DROP TABLE IF EXISTS backup_records;
