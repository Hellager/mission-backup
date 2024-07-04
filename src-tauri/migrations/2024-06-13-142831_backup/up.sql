-- Your SQL goes here
CREATE TABLE "backup" (
  "id"              INTEGER NOT NULL PRIMARY KEY,
  "backup_id"       TEXT NOT NULL UNIQUE,
  "mission_id"      TEXT NOT NULL,
  "save_path"       TEXT NOT NULL,
  "backup_size"     BIGINT NOT NULL,
  "reserved_0"       TEXT NOT NULL,
  "reserved_1"       TEXT NOT NULL,
  "reserved_2"       TEXT NOT NULL,
  "create_at"       TIMESTAMP NOT NULL,
  "update_at"       TIMESTAMP NOT NULL,
  "is_deleted"      SMALLINT NOT NULL,
  "delete_at"       TIMESTAMP NOT NULL
);
