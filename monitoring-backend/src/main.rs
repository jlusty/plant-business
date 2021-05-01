#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

// Exposes other files to be available
mod models;
mod schema;

use crate::models::{PlantMetricEntity, PlantMetricInsert};
use chrono::DateTime;
use diesel::prelude::*;
use rocket::{
    fairing::AdHoc,
    response::{status::Created, Debug},
    Build, Rocket,
};
use rocket_contrib::{databases::diesel as rocket_diesel, json::Json};
use schema::plant_metrics; // Used to get db schema

// Used to connect Rocket to the PostgreSQL database
#[database("postgres_timeseries")]
struct TimeseriesDbConn(rocket_diesel::PgConnection);

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
            .mount("/db", routes![post_timeseries, get_timeseries])
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(stage()).mount("/", routes![health])
}

#[get("/health")]
fn health() {}

#[get("/metric/<time_str>")]
async fn get_timeseries(
    conn: TimeseriesDbConn,
    time_str: String,
) -> Option<Json<PlantMetricEntity>> {
    let time = match DateTime::parse_from_rfc3339(&time_str) {
        Ok(t) => t,
        Err(_) => return None,
    };
    conn.run(move |conn| {
        plant_metrics::table
            .filter(plant_metrics::recorded_at.eq(time.naive_utc()))
            .first(conn)
    })
    .await
    .map(Json)
    .ok()
}

#[post("/metric", format = "json", data = "<metric>")]
async fn post_timeseries(
    metric: Json<PlantMetricInsert>,
    conn: TimeseriesDbConn,
) -> Result<Created<Json<PlantMetricEntity>>, Debug<diesel::result::Error>> {
    let metric_value = metric.clone();
    let r = conn
        .run(move |conn| {
            diesel::insert_into(plant_metrics::table)
                .values(&metric_value)
                .get_result(conn)
        })
        .await?;

    Ok(Created::new("/").body(Json(r)))
}
