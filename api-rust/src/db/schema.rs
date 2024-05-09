// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        content -> Nullable<Text>,
        status -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    profiles (id) {
        id -> Uuid,
        #[max_length = 255]
        first_name -> Nullable<Varchar>,
        #[max_length = 255]
        last_name -> Nullable<Varchar>,
        #[max_length = 255]
        full_name -> Nullable<Varchar>,
        #[max_length = 255]
        nickname -> Nullable<Varchar>,
        bio -> Nullable<Text>,
        birthday -> Nullable<Date>,
        #[max_length = 255]
        avatar -> Nullable<Varchar>,
        phone -> Nullable<Int8>,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    todos (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {

    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        encrypted_password -> Varchar,
        #[max_length = 255]
        reset_password_token -> Nullable<Varchar>,
        reset_password_sent_at -> Nullable<Timestamp>,
        remember_created_at -> Nullable<Timestamp>,
        sign_in_count -> Int4,
        current_sign_in_at -> Nullable<Timestamp>,
        last_sign_in_at -> Nullable<Timestamp>,
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_ip -> Nullable<Varchar>,
        status -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(profiles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    profiles,
    todos,
    users,
);
