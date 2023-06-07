// @generated automatically by Diesel CLI.

diesel::table! {
    todo (id) {
        id -> Nullable<Integer>,
        title -> Text,
        description -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user (id) {
        id -> Nullable<Integer>,
        email -> Text,
        password -> Text,
        name -> Text,
        age -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    todo,
    user,
);
