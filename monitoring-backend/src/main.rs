#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::databases::diesel;

#[database("postgres_timeseries")]
struct TimeseriesDbConn(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(TimeseriesDbConn::fairing())
        .mount("/", routes![index])
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
