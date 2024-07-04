-- Your SQL goes here
CREATE TABLE "ignore" (
  "id"              INTEGER NOT NULL PRIMARY KEY,
  "ignore_id"       TEXT NOT NULL UNIQUE,
  "procedure_id"    TEXT NOT NULL,
  "keyword"         TEXT NOT NULL,
  "reserved_0"       TEXT NOT NULL,
  "reserved_1"       TEXT NOT NULL,
  "reserved_2"       TEXT NOT NULL,
  "create_at"       TIMESTAMP NOT NULL,
  "update_at"       TIMESTAMP NOT NULL,
  "is_deleted"      SMALLINT NOT NULL,
  "delete_at"       TIMESTAMP NOT NULL
);
