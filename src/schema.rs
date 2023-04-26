// @generated automatically by Diesel CLI.

diesel::table! {
    collected_posts (id) {
        id -> Int4,
        user_id -> Int4,
        post_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        content -> Text,
        author_id -> Int4,
        post_id -> Int4,
        parent_comment_id -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        author_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    postscripts (id) {
        id -> Int4,
        post_id -> Int4,
        content -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        username -> Varchar,
        email_confirmed -> Bool,
        avatar_url -> Varchar,
        signature -> Varchar,
        role -> Varchar,
        last_login_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    collected_posts,
    comments,
    posts,
    postscripts,
    users,
);
