-- Your SQL goes here
CREATE TABLE ignore_keywords (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    keyword TEXT NOT NULL UNIQUE,
    pattern_type TEXT NOT NULL DEFAULT 'exact',
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    deleted_at TIMESTAMP
);

CREATE INDEX idx_ignore_keywords_keyword ON ignore_keywords(keyword) WHERE is_deleted = FALSE;

CREATE TRIGGER update_ignore_keywords_timestamp 
    AFTER UPDATE ON ignore_keywords
BEGIN
    UPDATE ignore_keywords SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;
