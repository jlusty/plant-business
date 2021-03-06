use crate::{
    models::{naive_date_time_serializer, PlantMetricEntity, PlantMetricInsert},
    schema::plant_metrics,
    TimeseriesDbConn,
};
use chrono::DateTime;
use diesel::{prelude::*, result::Error as DieselError};
use rocket::serde::json::Json;
use rocket::{
    response::{status::Created, Debug},
    Route,
};

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/", format = "json", data = "<metric>")]
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

#[get("/<id>")]
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

#[delete("/<id>")]
async fn delete_metric_by_id(id: i32, conn: TimeseriesDbConn) -> Result<Option<()>> {
    let affected = conn
        .run(move |conn| {
            diesel::delete(plant_metrics::table)
                .filter(plant_metrics::id.eq(id))
                .execute(conn)
        })
        .await?;

    Ok((affected == 1).then(|| ()))
}

#[get("/time/<time>")]
async fn get_metric_at_time(
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

#[get("/time/before/<time>")]
async fn get_metric_before_time(
    time: String,
    conn: TimeseriesDbConn,
) -> Option<Json<Vec<PlantMetricEntity>>> {
    let parsed_time = match DateTime::parse_from_rfc3339(&time) {
        Ok(t) => t,
        Err(_) => return None,
    };
    conn.run(move |conn| {
        plant_metrics::table
            .filter(plant_metrics::recorded_at.le(parsed_time.naive_utc()))
            .get_results(conn)
    })
    .await
    .map(Json)
    .ok()
}

#[delete("/time/before/<time>")]
async fn delete_metric_before_time(
    time: String,
    conn: TimeseriesDbConn,
) -> Result<Json<Vec<String>>> {
    let parsed_time = match DateTime::parse_from_rfc3339(&time) {
        Ok(t) => t,
        Err(_) => return Err(Debug::from(DieselError::NotFound)), // TODO: Handle error type better
    };
    let affected: Vec<PlantMetricEntity> = conn
        .run(move |conn| {
            diesel::delete(
                plant_metrics::table.filter(plant_metrics::recorded_at.le(parsed_time.naive_utc())),
            )
            .get_results(conn)
        })
        .await?;
    Ok(Json(
        affected
            .into_iter()
            .map(|metric| naive_date_time_serializer::get_datetime_string(&metric.recorded_at))
            .collect(),
    ))
}

pub fn routes() -> Vec<Route> {
    routes![
        post_metric,
        get_metric_by_id,
        delete_metric_by_id,
        get_metric_at_time,
        get_metric_before_time,
        delete_metric_before_time
    ]
}
