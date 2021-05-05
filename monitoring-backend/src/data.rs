use crate::{
    models::{DataEntry, TimeseriesData},
    schema::plant_metrics,
    TimeseriesDbConn,
};
use chrono::{DateTime, NaiveDateTime};
use diesel::prelude::*;
use rocket::Route;
use rocket_contrib::json::Json;

// TODO: Reduce code duplication
#[get("/temperature")]
async fn get_temperatures(conn: TimeseriesDbConn) -> Option<Json<TimeseriesData>> {
    let temperatures_vec: Vec<(NaiveDateTime, Option<f32>)> = conn
        .run(move |conn| {
            plant_metrics::table
                .filter(plant_metrics::temperature.is_not_null())
                .order(plant_metrics::recorded_at.asc())
                .select((plant_metrics::recorded_at, plant_metrics::temperature))
                .load::<(NaiveDateTime, Option<f32>)>(conn)
        })
        .await
        .ok()?;

    let temperatures = temperatures_vec
        .into_iter()
        .filter_map(|(time, temp)| temp.map(|data| DataEntry { time, data }))
        .collect();
    Some(Json(TimeseriesData {
        temperature: Some(temperatures),
        humidity: None,
        light: None,
        soil_moisture: None,
    }))
}

#[get("/temperature/<time>")]
async fn get_temperatures_after(
    time: String,
    conn: TimeseriesDbConn,
) -> Option<Json<TimeseriesData>> {
    let parsed_time = match DateTime::parse_from_rfc3339(&time) {
        Ok(t) => t,
        Err(_) => return None,
    };
    let temperatures_vec: Vec<(NaiveDateTime, Option<f32>)> = conn
        .run(move |conn| {
            plant_metrics::table
                .filter(plant_metrics::temperature.is_not_null())
                .filter(plant_metrics::recorded_at.gt(parsed_time.naive_utc()))
                .order(plant_metrics::recorded_at.asc())
                .select((plant_metrics::recorded_at, plant_metrics::temperature))
                .load::<(NaiveDateTime, Option<f32>)>(conn)
        })
        .await
        .ok()?;

    let temperatures = temperatures_vec
        .into_iter()
        .filter_map(|(time, temp)| temp.map(|data| DataEntry { time, data }))
        .collect();
    Some(Json(TimeseriesData {
        temperature: Some(temperatures),
        humidity: None,
        light: None,
        soil_moisture: None,
    }))
}

#[get("/humidity")]
async fn get_humidity(conn: TimeseriesDbConn) -> Option<Json<TimeseriesData>> {
    let humidity_vec: Vec<(NaiveDateTime, Option<f32>)> = conn
        .run(move |conn| {
            plant_metrics::table
                .filter(plant_metrics::humidity.is_not_null())
                .order(plant_metrics::recorded_at.asc())
                .select((plant_metrics::recorded_at, plant_metrics::humidity))
                .load::<(NaiveDateTime, Option<f32>)>(conn)
        })
        .await
        .ok()?;

    let humidity = humidity_vec
        .into_iter()
        .filter_map(|(time, temp)| temp.map(|data| DataEntry { time, data }))
        .collect();
    Some(Json(TimeseriesData {
        temperature: None,
        humidity: Some(humidity),
        light: None,
        soil_moisture: None,
    }))
}

#[get("/humidity/<time>")]
async fn get_humidity_after(time: String, conn: TimeseriesDbConn) -> Option<Json<TimeseriesData>> {
    let parsed_time = match DateTime::parse_from_rfc3339(&time) {
        Ok(t) => t,
        Err(_) => return None,
    };
    let humidity_vec: Vec<(NaiveDateTime, Option<f32>)> = conn
        .run(move |conn| {
            plant_metrics::table
                .filter(plant_metrics::humidity.is_not_null())
                .filter(plant_metrics::recorded_at.gt(parsed_time.naive_utc()))
                .order(plant_metrics::recorded_at.asc())
                .select((plant_metrics::recorded_at, plant_metrics::humidity))
                .load::<(NaiveDateTime, Option<f32>)>(conn)
        })
        .await
        .ok()?;

    let humidity = humidity_vec
        .into_iter()
        .filter_map(|(time, temp)| temp.map(|data| DataEntry { time, data }))
        .collect();
    Some(Json(TimeseriesData {
        temperature: None,
        humidity: Some(humidity),
        light: None,
        soil_moisture: None,
    }))
}

#[get("/light")]
async fn get_light(conn: TimeseriesDbConn) -> Option<Json<TimeseriesData>> {
    let light_vec: Vec<(NaiveDateTime, Option<i32>)> = conn
        .run(move |conn| {
            plant_metrics::table
                .filter(plant_metrics::light.is_not_null())
                .order(plant_metrics::recorded_at.asc())
                .select((plant_metrics::recorded_at, plant_metrics::light))
                .load::<(NaiveDateTime, Option<i32>)>(conn)
        })
        .await
        .ok()?;

    let light = light_vec
        .into_iter()
        .filter_map(|(time, temp)| temp.map(|data| DataEntry { time, data }))
        .collect();
    Some(Json(TimeseriesData {
        temperature: None,
        humidity: None,
        light: Some(light),
        soil_moisture: None,
    }))
}

#[get("/light/<time>")]
async fn get_light_after(time: String, conn: TimeseriesDbConn) -> Option<Json<TimeseriesData>> {
    let parsed_time = match DateTime::parse_from_rfc3339(&time) {
        Ok(t) => t,
        Err(_) => return None,
    };
    let light_vec: Vec<(NaiveDateTime, Option<i32>)> = conn
        .run(move |conn| {
            plant_metrics::table
                .filter(plant_metrics::light.is_not_null())
                .filter(plant_metrics::recorded_at.gt(parsed_time.naive_utc()))
                .order(plant_metrics::recorded_at.asc())
                .select((plant_metrics::recorded_at, plant_metrics::light))
                .load::<(NaiveDateTime, Option<i32>)>(conn)
        })
        .await
        .ok()?;

    let light = light_vec
        .into_iter()
        .filter_map(|(time, temp)| temp.map(|data| DataEntry { time, data }))
        .collect();
    Some(Json(TimeseriesData {
        temperature: None,
        humidity: None,
        light: Some(light),
        soil_moisture: None,
    }))
}

#[get("/soilmoisture")]
async fn get_soil_moisture(conn: TimeseriesDbConn) -> Option<Json<TimeseriesData>> {
    let soil_moisture_vec: Vec<(NaiveDateTime, Option<i32>)> = conn
        .run(move |conn| {
            plant_metrics::table
                .filter(plant_metrics::soil_moisture.is_not_null())
                .order(plant_metrics::recorded_at.asc())
                .select((plant_metrics::recorded_at, plant_metrics::soil_moisture))
                .load::<(NaiveDateTime, Option<i32>)>(conn)
        })
        .await
        .ok()?;

    let soil_moisture = soil_moisture_vec
        .into_iter()
        .filter_map(|(time, temp)| temp.map(|data| DataEntry { time, data }))
        .collect();
    Some(Json(TimeseriesData {
        temperature: None,
        humidity: None,
        light: None,
        soil_moisture: Some(soil_moisture),
    }))
}

#[get("/soilmoisture/<time>")]
async fn get_soil_moisture_after(
    time: String,
    conn: TimeseriesDbConn,
) -> Option<Json<TimeseriesData>> {
    let parsed_time = match DateTime::parse_from_rfc3339(&time) {
        Ok(t) => t,
        Err(_) => return None,
    };
    let soil_moisture_vec: Vec<(NaiveDateTime, Option<i32>)> = conn
        .run(move |conn| {
            plant_metrics::table
                .filter(plant_metrics::soil_moisture.is_not_null())
                .filter(plant_metrics::recorded_at.gt(parsed_time.naive_utc()))
                .order(plant_metrics::recorded_at.asc())
                .select((plant_metrics::recorded_at, plant_metrics::soil_moisture))
                .load::<(NaiveDateTime, Option<i32>)>(conn)
        })
        .await
        .ok()?;

    let soil_moisture = soil_moisture_vec
        .into_iter()
        .filter_map(|(time, temp)| temp.map(|data| DataEntry { time, data }))
        .collect();
    Some(Json(TimeseriesData {
        temperature: None,
        humidity: None,
        light: None,
        soil_moisture: Some(soil_moisture),
    }))
}

pub fn routes() -> Vec<Route> {
    routes![
        get_temperatures,
        get_temperatures_after,
        get_humidity,
        get_humidity_after,
        get_light,
        get_light_after,
        get_soil_moisture,
        get_soil_moisture_after,
    ]
}
