#[macro_use]
extern crate diesel;

use figment::{providers::Env, Figment, Profile};

use rocket::{get, launch, routes};
use rocket_sync_db_pools::{database, diesel as d};

mod models;
mod schema;
pub mod session;

#[database("secrust_santa")]
pub struct SecrustSantaDbConn(pub d::PgConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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
            routes![index, session::create, session::list, session::get],
        )
}
