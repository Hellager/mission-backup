-- Your SQL goes here
CREATE TABLE "mission" (
  "id"                  INTEGER NOT NULL PRIMARY KEY,
  "mission_id"          TEXT NOT NULL UNIQUE,
  "procedure_id"        TEXT NOT NULL,
  "name"                TEXT NOT NULL,
  "status"              SMALLINT NOT NULL,
  "description"         TEXT NOT NULL,
  "src_path"            TEXT NOT NULL,
  "dst_path"            TEXT NOT NULL,
  "path_type"           SMALLINT NOT NULL,
  "next_runtime"        TIMESTAMP NOT NULL,
  "last_trigger"        TIMESTAMP NOT NULL,
  "reserved_0"           TEXT NOT NULL,
  "reserved_1"           TEXT NOT NULL,
  "reserved_2"           TEXT NOT NULL,
  "create_at"           TIMESTAMP NOT NULL,
  "update_at"           TIMESTAMP NOT NULL,
  "is_deleted"          SMALLINT NOT NULL,
  "delete_at"           TIMESTAMP NOT NULL
);
