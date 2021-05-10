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
use rocket::{fairing::AdHoc, http::Method, Build, Rocket};
use rocket_contrib::{databases::diesel as rocket_diesel, serve::StaticFiles};
use rocket_cors::{Cors, CorsOptions};

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

pub fn cors() -> Cors {
    CorsOptions::default()
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .to_cors()
        .expect("Failed to initialise CORS")
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let rocket = rocket::build()
        .attach(stage())
        .attach(cors())
        .mount("/health", routes![health]);

    let static_files_path_param: String = rocket
        .figment()
        .extract_inner("static_files_path")
        .expect("Static files path not passed");
    let static_files_path = match static_files_path_param
        .chars()
        .next()
        .expect("static_files_path should not be empty")
    {
        '/' => static_files_path_param,
        _ => {
            // Find static files relative to crate
            // Reimplement crate_relative! macro manually as it doesn't support dynamic strings
            if cfg!(windows) {
                format!(
                    "{}\\{}",
                    env!("CARGO_MANIFEST_DIR"),
                    static_files_path_param
                )
            } else {
                format!("{}/{}", env!("CARGO_MANIFEST_DIR"), static_files_path_param)
            }
        }
    };
    rocket.mount("/", StaticFiles::from(static_files_path))
}

#[get("/")]
fn health() {}
