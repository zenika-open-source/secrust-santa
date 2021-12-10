#[macro_use]
extern crate diesel;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use figment::{providers::Env, Figment, Profile};

use rocket::{
    get,
    http::Status,
    launch, put,
    response::status::{Created, NotFound},
    routes,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_sync_db_pools::{database, diesel as d};
use uuid::Uuid;

mod models;
mod schema;

#[database("secrust_santa")]
struct SecrustSantaDbConn(d::PgConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct SessionForm {
    name: String,
}

#[derive(Default, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct SessionResponse {
    id: Uuid,
    name: String,
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
async fn create_session(
    conn: SecrustSantaDbConn,
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
async fn get_sessions(conn: SecrustSantaDbConn) -> Result<Json<Vec<SessionResponse>>, Status> {
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
async fn get_session(
    conn: SecrustSantaDbConn,
    session_id: Uuid,
) -> Result<Json<SessionResponse>, NotFound<String>> {
    use self::schema::sessions::dsl::*;
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

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().unwrap();
    let figment = Figment::from(rocket::Config::default())
        .merge(Env::prefixed("APP_").global())
        .select(Profile::Default);
    rocket::custom(figment)
        .attach(SecrustSantaDbConn::fairing())
        .mount(
            "/",
            routes![index, create_session, get_sessions, get_session],
        )
}
