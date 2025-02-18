// @generated automatically by Diesel CLI.

diesel::table! {
    backup_records (id) {
        id -> Integer,
        mission_id -> Integer,
        backup_path -> Text,
        file_size -> BigInt,
        file_hash -> Nullable<Text>,
        status -> Text,
        error_message -> Nullable<Text>,
        compression_format -> Nullable<Text>,
        compression_level -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_deleted -> Bool,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    backup_settings (id) {
        id -> Integer,
        name -> Text,
        ignore_enabled -> Bool,
        ignore_patterns -> Text,
        compression_enabled -> Bool,
        compression_format -> Text,
        compression_level -> Integer,
        trigger_type -> Text,
        cron_expression -> Nullable<Text>,
        backup_limit_enabled -> Bool,
        max_backup_count -> Nullable<Integer>,
        max_backup_size -> Nullable<BigInt>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_deleted -> Bool,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    db_version (id) {
        id -> Integer,
        version -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_deleted -> Bool,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    ignore_keywords (id) {
        id -> Integer,
        keyword -> Text,
        pattern_type -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_deleted -> Bool,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    ignore_settings_relations (id) {
        id -> Integer,
        backup_setting_id -> Integer,
        ignore_keyword_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_deleted -> Bool,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    mission_backup_relations (id) {
        id -> Integer,
        mission_id -> Integer,
        backup_setting_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_deleted -> Bool,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    mission_settings (id) {
        id -> Integer,
        name -> Text,
        status -> Text,
        description -> Nullable<Text>,
        source_path -> Text,
        target_path -> Text,
        object_type -> Text,
        next_run_time -> Nullable<Timestamp>,
        last_trigger_time -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_deleted -> Bool,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(backup_records -> mission_settings (mission_id));
diesel::joinable!(ignore_settings_relations -> backup_settings (backup_setting_id));
diesel::joinable!(ignore_settings_relations -> ignore_keywords (ignore_keyword_id));
diesel::joinable!(mission_backup_relations -> backup_settings (backup_setting_id));
diesel::joinable!(mission_backup_relations -> mission_settings (mission_id));

diesel::allow_tables_to_appear_in_same_query!(
    backup_records,
    backup_settings,
    db_version,
    ignore_keywords,
    ignore_settings_relations,
    mission_backup_relations,
    mission_settings,
);
