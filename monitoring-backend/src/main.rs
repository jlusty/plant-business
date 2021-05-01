#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel_migrations;

use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use rocket_contrib::databases::diesel;

// Used to connect Rocket to the PostgreSQL database
#[database("postgres_timeseries")]
struct TimeseriesDbConn(diesel::PgConnection);

// Embeds the SQL migrations in the binary
async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    // Path to migrations folder
    embed_migrations!("migrations");

    let conn = TimeseriesDbConn::get_one(&rocket)
        .await
        .expect("Database connection failed");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("Diesel migrations failed");

    rocket
}

// Connect Rocket to the to the PostgreSQL database
// Runs the migrations, then exposes endpoints connected to the database
// This should be run on start-up
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel PostgreSQL initialisation", |rocket| async {
        rocket
            .attach(TimeseriesDbConn::fairing())
            .attach(AdHoc::on_ignite("Run diesel migrations", run_migrations))
            .mount("/db", routes![get_timeseries])
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(stage()).mount("/", routes![index])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/timeseries/<id>")]
async fn get_timeseries(conn: TimeseriesDbConn, id: usize) -> Logs {
    conn.run(|c| timeseries::filter(id.eq(log_id)).load(c))
        .await
}
