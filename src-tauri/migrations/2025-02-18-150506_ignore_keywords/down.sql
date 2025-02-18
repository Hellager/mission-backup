-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS update_ignore_keywords_timestamp;
DROP INDEX IF EXISTS idx_ignore_keywords_keyword;
DROP TABLE IF EXISTS ignore_keywords;
