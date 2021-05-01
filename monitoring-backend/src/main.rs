#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

// Exposes schema file to be available here
pub mod schema;

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use rocket::{fairing::AdHoc, response::status::Created, response::Debug, Build, Rocket};
use rocket_contrib::{databases::diesel as rocket_diesel, json::Json};
use schema::plant_metrics; // Used to get db schema
use serde::{Deserialize, Serialize};

// Model type used to interact with DBO
// Queryable: Allows diesel to get this type back when querying the DB
// Serialize: Allows serde to serialize the database entry to JSON of this type
#[derive(Queryable, Serialize)]
pub struct PlantMetricEntity {
    #[serde(with = "my_date_format")]
    pub recorded_at: NaiveDateTime,
    pub temperature: Option<f32>,
    pub humidity: Option<f32>,
    pub light: Option<i32>,
    pub soil_moisture: Option<i32>,
}

mod my_date_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(
        time: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let s = DateTime::<Utc>::from_utc(*time, Utc).to_rfc3339();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error> {
        let time: String = Deserialize::deserialize(deserializer)?;
        Ok(DateTime::parse_from_rfc3339(&time)
            .map_err(serde::de::Error::custom)?
            .naive_utc())
    }
}

fn time_now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

// Insertable: Allows diesel to use this type when inserting into the DB
// Deserialize: Allows serde to deserialize request body to this type
#[derive(Clone, Insertable, Deserialize)]
#[table_name = "plant_metrics"] // Table where to insert
#[serde(deny_unknown_fields)] // Fails if extra fields provided in JSON body
pub struct PlantMetricInsert {
    // When deserialising, add the time now as the recorded at time
    #[serde(skip_deserializing, default = "time_now")]
    pub recorded_at: NaiveDateTime,
    pub temperature: f32,
    pub humidity: f32,
    pub light: i32,
    pub soil_moisture: i32,
}

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
    rocket::build().attach(stage()).mount("/", routes![index])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

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
