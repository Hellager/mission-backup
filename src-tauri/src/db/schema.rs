// @generated automatically by Diesel CLI.

diesel::table! {
    backup (id) {
        id -> Integer,
        backup_id -> Text,
        mission_id -> Text,
        save_path -> Text,
        backup_size -> BigInt,
        reserved_0 -> Text,
        reserved_1 -> Text,
        reserved_2 -> Text,
        create_at -> Timestamp,
        update_at -> Timestamp,
        is_deleted -> SmallInt,
        delete_at -> Timestamp,
    }
}

diesel::table! {
    ignore (id) {
        id -> Integer,
        ignore_id -> Text,
        procedure_id -> Text,
        keyword -> Text,
        reserved_0 -> Text,
        reserved_1 -> Text,
        reserved_2 -> Text,
        create_at -> Timestamp,
        update_at -> Timestamp,
        is_deleted -> SmallInt,
        delete_at -> Timestamp,
    }
}

diesel::table! {
    mission (id) {
        id -> Integer,
        mission_id -> Text,
        procedure_id -> Text,
        name -> Text,
        status -> SmallInt,
        description -> Text,
        src_path -> Text,
        dst_path -> Text,
        path_type -> SmallInt,
        next_runtime -> Timestamp,
        last_trigger -> Timestamp,
        reserved_0 -> Text,
        reserved_1 -> Text,
        reserved_2 -> Text,
        create_at -> Timestamp,
        update_at -> Timestamp,
        is_deleted -> SmallInt,
        delete_at -> Timestamp,
    }
}

diesel::table! {
    procedure (id) {
        id -> Integer,
        procedure_id -> Text,
        name -> Text,
        has_ignores -> Bool,
        ignore_method -> SmallInt,
        is_compress -> Bool,
        compress_format -> SmallInt,
        trigger -> SmallInt,
        cron_expression -> Text,
        restrict -> SmallInt,
        restrict_days -> SmallInt,
        restrict_size -> BigInt,
        reserved_0 -> Text,
        reserved_1 -> Text,
        reserved_2 -> Text,
        create_at -> Timestamp,
        update_at -> Timestamp,
        is_deleted -> SmallInt,
        delete_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    backup,
    ignore,
    mission,
    procedure,
);
