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
