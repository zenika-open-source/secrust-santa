#[macro_use]
extern crate diesel;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use figment::{providers::Env, Figment, Profile};

use rocket::{
    get, launch, post,
    response::status::{Conflict, Created},
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

#[post("/session", data = "<session_form>")]
async fn create_session(
    conn: SecrustSantaDbConn,
    session_form: Json<SessionForm>,
) -> Result<Created<Json<SessionResponse>>, Conflict<String>> {
    let session_form = session_form.into_inner();
    let session: models::Session = conn
        .run(|c| {
            diesel::insert_into(schema::sessions::table)
                .values(models::NewSession::from(session_form))
                .get_result::<models::Session>(c)
                .expect("Error saving new post")
        })
        .await;
    Ok(Created::new("").body(Json(SessionResponse::from(session))))
}

#[get("/session")]
async fn get_sessions(conn: SecrustSantaDbConn) -> Json<Vec<SessionResponse>> {
    let sessions: Vec<models::Session> = conn
        .run(|c| {
            schema::sessions::table
                .load::<models::Session>(c)
                .expect("Error loading sessions")
        })
        .await;
    Json(sessions.into_iter().map(SessionResponse::from).collect())
}

#[get("/session/<session_id>")]
async fn get_session(conn: SecrustSantaDbConn, session_id: Uuid) -> Json<SessionResponse> {
    use self::schema::sessions::dsl::*;
    let session: models::Session = conn
        .run(move |c| {
            sessions
                .find(session_id)
                .get_result(c)
                .expect("Unable to load session")
        })
        .await;
    Json(SessionResponse::from(session))
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
