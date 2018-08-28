table! {
    parties (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
    }
}

table! {
    people (id) {
        id -> Int4,
        party -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        phone_number -> Nullable<Text>,
    }
}

table! {
    users (user_id) {
        user_id -> Uuid,
        username -> Varchar,
        is_admin -> Bool,
        hash -> Text,
    }
}

joinable!(people -> parties (party));

allow_tables_to_appear_in_same_query!(
    parties,
    people,
    users,
);
