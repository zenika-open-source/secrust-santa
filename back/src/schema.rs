table! {
    blacklisted (user1_id, user2_id) {
        session_id -> Uuid,
        user1_id -> Uuid,
        user2_id -> Uuid,
    }
}

table! {
    sessions (id) {
        id -> Uuid,
        name -> Text,
    }
}

table! {
    tossed (user1_id, user2_id) {
        session_id -> Uuid,
        user1_id -> Uuid,
        user2_id -> Uuid,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        session_id -> Uuid,
    }
}

joinable!(blacklisted -> sessions (session_id));
joinable!(tossed -> sessions (session_id));
joinable!(users -> sessions (session_id));

allow_tables_to_appear_in_same_query!(
    blacklisted,
    sessions,
    tossed,
    users,
);
