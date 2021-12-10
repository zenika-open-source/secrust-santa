use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use crate::models;
use crate::schema;
use rocket::{
    get,
    http::Status,
    put,
    response::status::{Created, NotFound},
    serde::{json::Json, Deserialize, Serialize},
};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SessionForm {
    pub name: String,
}

#[derive(Default, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SessionResponse {
    pub id: Uuid,
    pub name: String,
}

impl From<models::Session> for SessionResponse {
    fn from(session: models::Session) -> Self {
        Self {
            id: session.id,
            name: session.name.to_owned(),
        }
    }
}

impl SessionResponse {
    fn get_url(&self) -> String {
        format!("/session/{}", self.id)
    }
}

#[put("/session", data = "<session_form>")]
pub async fn create(
    conn: crate::SecrustSantaDbConn,
    session_form: Json<SessionForm>,
) -> Result<Created<Json<SessionResponse>>, Status> {
    let session_form = session_form.into_inner();
    let session: Result<models::Session, diesel::result::Error> = conn
        .run(|c| {
            diesel::insert_into(schema::sessions::table)
                .values(models::NewSession::from(session_form))
                .get_result::<models::Session>(c)
        })
        .await;

    match session {
        Ok(session) => {
            let session = SessionResponse::from(session);
            Ok(Created::new(session.get_url()).body(Json(SessionResponse::from(session))))
        }
        Err(_e) => Err(Status::InternalServerError),
    }
}

#[get("/session")]
pub async fn list(conn: crate::SecrustSantaDbConn) -> Result<Json<Vec<SessionResponse>>, Status> {
    conn.run(|c| schema::sessions::table.load::<models::Session>(c))
        .await
        .and_then(|sessions| {
            Ok(Json(
                sessions.into_iter().map(SessionResponse::from).collect(),
            ))
        })
        .or_else(|_| Err(Status::NotFound))
}

#[get("/session/<session_id>")]
pub async fn get(
    conn: crate::SecrustSantaDbConn,
    session_id: Uuid,
) -> Result<Json<SessionResponse>, NotFound<String>> {
    use crate::schema::sessions::dsl::*;
    let session: Result<models::Session, diesel::result::Error> = conn
        .run(move |c| sessions.find(session_id).get_result(c))
        .await;

    session
        .and_then(|session| Ok(Json(SessionResponse::from(session))))
        .or_else(|_| {
            Err(NotFound(format!(
                "Session not found for uuid: {}",
                session_id
            )))
        })
}
