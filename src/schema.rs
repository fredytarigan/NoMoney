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

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 64]
        first_name -> Nullable<Varchar>,
        #[max_length = 64]
        last_name -> Nullable<Varchar>,
        #[max_length = 128]
        password -> Varchar,
        active -> Bool,
        family_id -> Uuid,
        role_id -> Uuid,
        #[max_length = 64]
        email -> Varchar,
        email_validated -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(users -> families (family_id));
diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(families, roles, users,);
