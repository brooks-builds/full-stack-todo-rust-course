// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        priority -> Nullable<Varchar>,
        title -> Varchar,
        completed_at -> Nullable<Timestamp>,
        description -> Nullable<Text>,
        deleted_at -> Nullable<Timestamp>,
        user_id -> Nullable<Int4>,
        is_default -> Nullable<Bool>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        deleted_at -> Nullable<Timestamp>,
        token -> Nullable<Text>,
    }
}

diesel::joinable!(tasks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
