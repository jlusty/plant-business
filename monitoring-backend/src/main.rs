#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;

// Exposes other files to be available
mod data;
mod metric;
mod models;
mod schema;

use dotenv::dotenv;
use rocket::{fairing::AdHoc, Build, Rocket};
use rocket_contrib::databases::diesel as rocket_diesel;

// Used to connect Rocket to the PostgreSQL database
#[database("postgres_timeseries")]
pub struct TimeseriesDbConn(rocket_diesel::PgConnection);

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
            .mount("/db/data", data::routes())
            .mount("/db/metric", metric::routes())
    })
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().attach(stage()).mount("/", routes![health])
}

#[get("/health")]
fn health() {}
