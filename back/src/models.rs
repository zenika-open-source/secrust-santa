use crate::schema::sessions;
use crate::SessionForm;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Session {
    pub id: Uuid,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "sessions"]
pub struct NewSession {
    pub name: String,
}

impl From<SessionForm> for NewSession {
    fn from(form: SessionForm) -> Self {
        NewSession { name: form.name }
    }
}

#[derive(Queryable, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[derive(Queryable, Debug)]
pub struct Blacklisted {
    pub session_id: Uuid,
    pub user1_id: Uuid,
    pub user2_id: Uuid,
}

#[derive(Queryable, Debug)]
pub struct Tossed {
    pub session_id: Uuid,
    pub user1_id: Uuid,
    pub user2_id: Uuid,
}
