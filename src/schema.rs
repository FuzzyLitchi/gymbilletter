table! {
    parties (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
    }
}

table! {
    users (user_id) {
        user_id -> Uuid,
        username -> Varchar,
        is_admin -> Bool,
        salt_hash -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    parties,
    users,
);
