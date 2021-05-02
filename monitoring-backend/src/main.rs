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
mod models;
mod schema;

use crate::models::{PlantMetricEntity, PlantMetricInsert, TimeseriesData};
use chrono::{DateTime, NaiveDateTime};
use diesel::prelude::*;
use dotenv::dotenv;
use rocket::{
    fairing::AdHoc,
    response::{status::Created, Debug},
    Build, Rocket,
};
use rocket_contrib::{databases::diesel as rocket_diesel, json::Json};
use schema::plant_metrics; // Used to get db schema

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

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
            .mount(
                "/db",
                routes![
                    post_metric,
                    get_metric_by_id,
                    get_metric_by_time,
                    get_temperatures,
                    get_humidity,
                    get_light,
                    get_soil_moisture
                ],
            )
    })
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().attach(stage()).mount("/", routes![health])
}

#[get("/health")]
fn health() {}

#[post("/metric", format = "json", data = "<metric>")]
async fn post_metric(
    metric: Json<PlantMetricInsert>,
    conn: TimeseriesDbConn,
) -> Result<Created<Json<PlantMetricEntity>>> {
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

#[get("/metric/<id>")]
async fn get_metric_by_id(id: i32, conn: TimeseriesDbConn) -> Option<Json<PlantMetricEntity>> {
    conn.run(move |conn| {
        plant_metrics::table
            .filter(plant_metrics::id.eq(id))
            .first(conn)
    })
    .await
    .map(Json)
    .ok()
}

#[get("/metric/time/<time>")]
async fn get_metric_by_time(
    time: String,
    conn: TimeseriesDbConn,
) -> Option<Json<PlantMetricEntity>> {
    let parsed_time = match DateTime::parse_from_rfc3339(&time) {
        Ok(t) => t,
        Err(_) => return None,
    };
    conn.run(move |conn| {
        plant_metrics::table
            .filter(plant_metrics::recorded_at.eq(parsed_time.naive_utc()))
            .first(conn)
    })
    .await
    .map(Json)
    .ok()
}

// TODO: Reduce code duplication
#[get("/data/temperature")]
async fn get_temperatures(conn: TimeseriesDbConn) -> Option<Json<TimeseriesData>> {
    let temperatures_vec: Vec<(NaiveDateTime, Option<f32>)> = conn
        .run(move |conn| {
            plant_metrics::table
                .filter(plant_metrics::temperature.is_not_null())
                .order(plant_metrics::recorded_at.desc())
                .select((plant_metrics::recorded_at, plant_metrics::temperature))
                .load::<(NaiveDateTime, Option<f32>)>(conn)
        })
        .await
        .ok()?;

    let temperatures = temperatures_vec
        .into_iter()
        .filter_map(|(time, temp)| temp.map(|t| (time, t)))
        .collect();
    Some(Json(TimeseriesData {
        temperature: Some(temperatures),
        humidity: None,
        light: None,
        soil_moisture: None,
    }))
}

#[get("/data/humidity")]
async fn get_humidity(conn: TimeseriesDbConn) -> Option<Json<TimeseriesData>> {
    let humidity_vec: Vec<(NaiveDateTime, Option<f32>)> = conn
        .run(move |conn| {
            plant_metrics::table
                .filter(plant_metrics::humidity.is_not_null())
                .order(plant_metrics::recorded_at.desc())
                .select((plant_metrics::recorded_at, plant_metrics::humidity))
                .load::<(NaiveDateTime, Option<f32>)>(conn)
        })
        .await
        .ok()?;

    let humidity = humidity_vec
        .into_iter()
        .filter_map(|(time, temp)| temp.map(|t| (time, t)))
        .collect();
    Some(Json(TimeseriesData {
        temperature: None,
        humidity: Some(humidity),
        light: None,
        soil_moisture: None,
    }))
}

#[get("/data/light")]
async fn get_light(conn: TimeseriesDbConn) -> Option<Json<TimeseriesData>> {
    let light_vec: Vec<(NaiveDateTime, Option<i32>)> = conn
        .run(move |conn| {
            plant_metrics::table
                .filter(plant_metrics::light.is_not_null())
                .order(plant_metrics::recorded_at.desc())
                .select((plant_metrics::recorded_at, plant_metrics::light))
                .load::<(NaiveDateTime, Option<i32>)>(conn)
        })
        .await
        .ok()?;

    let light = light_vec
        .into_iter()
        .filter_map(|(time, temp)| temp.map(|t| (time, t)))
        .collect();
    Some(Json(TimeseriesData {
        temperature: None,
        humidity: None,
        light: Some(light),
        soil_moisture: None,
    }))
}

#[get("/data/soilmoisture")]
async fn get_soil_moisture(conn: TimeseriesDbConn) -> Option<Json<TimeseriesData>> {
    let soil_moisture_vec: Vec<(NaiveDateTime, Option<i32>)> = conn
        .run(move |conn| {
            plant_metrics::table
                .filter(plant_metrics::soil_moisture.is_not_null())
                .order(plant_metrics::recorded_at.desc())
                .select((plant_metrics::recorded_at, plant_metrics::soil_moisture))
                .load::<(NaiveDateTime, Option<i32>)>(conn)
        })
        .await
        .ok()?;

    let soil_moisture = soil_moisture_vec
        .into_iter()
        .filter_map(|(time, temp)| temp.map(|t| (time, t)))
        .collect();
    Some(Json(TimeseriesData {
        temperature: None,
        humidity: None,
        light: None,
        soil_moisture: Some(soil_moisture),
    }))
}
