#[macro_use]
extern crate diesel;

use rocket::{get, launch, routes};

mod models;
mod schema;

use rocket_sync_db_pools::{database, diesel as d};

#[database("secrust_santa")]
struct LogsDbConn(d::PgConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(LogsDbConn::fairing())
        .mount("/", routes![index])
}
