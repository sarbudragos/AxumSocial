// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        #[max_length = 255]
        content -> Varchar,
        creation_date -> Timestamptz,
        user_id -> Int4,
    }
}

diesel::table! {
    user_follows (follower_id, following_user_id) {
        follower_id -> Int4,
        following_user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        email -> Varchar,
    }
}

diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    user_follows,
    users,
);
