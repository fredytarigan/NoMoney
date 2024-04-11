// @generated automatically by Diesel CLI.

diesel::table! {
    families (id) {
        id -> Uuid,
        #[max_length = 64]
        name -> Varchar,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        #[max_length = 64]
        name -> Varchar,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    families,
    roles,
);
