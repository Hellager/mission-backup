-- Your SQL goes here
CREATE TABLE "procedure" (
  "id"                      INTEGER NOT NULL PRIMARY KEY,
  "procedure_id"            TEXT NOT NULL UNIQUE,
  "name"                    TEXT NOT NULL,
  "has_ignores"             BOOL NOT NULL,
  "ignore_method"           SMALLINT NOT NULL,
  "is_compress"             BOOL NOT NULL,
  "compress_format"         SMALLINT NOT NULL,
  "trigger"                 SMALLINT NOT NULL,
  "cron_expression"         TEXT NOT NULL,
  "restrict"                SMALLINT NOT NULL,
  "restrict_days"           SMALLINT NOT NULL,
  "restrict_size"           BIGINT NOT NULL,
  "reserved_0"               TEXT NOT NULL,
  "reserved_1"               TEXT NOT NULL,
  "reserved_2"               TEXT NOT NULL,
  "create_at"               TIMESTAMP NOT NULL,
  "update_at"               TIMESTAMP NOT NULL,
  "is_deleted"              SMALLINT NOT NULL,
  "delete_at"               TIMESTAMP NOT NULL
);
