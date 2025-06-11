// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        title -> Varchar,
        content -> Text,
        user_id -> Uuid,
        excerpt -> Nullable<Varchar>,
        slug -> Varchar,
        published -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(users, posts,);
