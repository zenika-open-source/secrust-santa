use diesel::pg::types::sql_types::Uuid;

#[derive(Queryable)]
pub struct Session {
    pub id: Uuid,
    pub name: String,
}

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[derive(Queryable)]
pub struct Blacklisted {
    pub session_id: Uuid,
    pub user1_id: Uuid,
    pub user2_id: Uuid,
}

#[derive(Queryable)]
pub struct Tossed {
    pub session_id: Uuid,
    pub user1_id: Uuid,
    pub user2_id: Uuid,
}
