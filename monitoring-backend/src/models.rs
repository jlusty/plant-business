use super::schema::plant_metrics; // Used to get db schema
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

// Model type used to interact with DBO
// Queryable: Allows diesel to get this type back when querying the DB
// Serialize: Allows serde to serialize the database entry to JSON of this type
#[derive(Queryable, Serialize)]
pub struct PlantMetricEntity {
    pub id: i32,
    #[serde(with = "naive_date_time_serializer")]
    pub recorded_at: NaiveDateTime,
    pub temperature: Option<f32>,
    pub humidity: Option<f32>,
    pub light: Option<i32>,
    pub soil_moisture: Option<i32>,
}

// Insertable: Allows diesel to use this type when inserting into the DB
// Deserialize: Allows serde to deserialize request body to this type
#[derive(Clone, Insertable, Deserialize)]
#[table_name = "plant_metrics"] // Table where to insert
#[serde(deny_unknown_fields)] // Fails if extra fields provided in JSON body
pub struct PlantMetricInsert {
    pub temperature: f32,
    pub humidity: f32,
    pub light: i32,
    pub soil_moisture: i32,
}

pub mod naive_date_time_serializer {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::Serializer;

    pub fn get_datetime_string(time: &NaiveDateTime) -> String {
        DateTime::<Utc>::from_utc(*time, Utc).to_rfc3339()
    }

    pub fn serialize<S: Serializer>(
        time: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let s = get_datetime_string(time);
        serializer.serialize_str(&s)
    }
}

#[derive(Serialize)]
pub struct TimeseriesData {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "timeseries_map")]
    pub temperature: Option<HashMap<NaiveDateTime, f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "timeseries_map")]
    pub humidity: Option<HashMap<NaiveDateTime, f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "timeseries_map")]
    pub light: Option<HashMap<NaiveDateTime, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "timeseries_map")]
    pub soil_moisture: Option<HashMap<NaiveDateTime, i32>>,
}

pub mod timeseries_map {
    use super::naive_date_time_serializer;
    use chrono::NaiveDateTime;
    use serde::{ser::SerializeMap, Serialize, Serializer};
    use std::collections::HashMap;

    pub fn serialize<S: Serializer, V: Serialize>(
        timemap_opt: &Option<HashMap<NaiveDateTime, V>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match timemap_opt {
            Some(timemap) => {
                let mut map = serializer.serialize_map(Some(timemap.len()))?;
                for (k, v) in timemap {
                    let s = naive_date_time_serializer::get_datetime_string(k);
                    map.serialize_entry(&s, &v)?;
                }
                map.end()
            }
            None => {
                // Serialise to empty map
                serializer.serialize_map(Some(0))?.end()
            }
        }
    }
}
