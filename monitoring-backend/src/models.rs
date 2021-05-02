use super::schema::plant_metrics; // Used to get db schema
use chrono::NaiveDateTime;
use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};

use std::collections::HashMap;

// Model type used to interact with DBO
// Queryable: Allows diesel to get this type back when querying the DB
// Serialize: Allows serde to serialize the database entry to JSON of this type
#[derive(Queryable, Serialize)]
pub struct PlantMetricEntity {
    pub id: i32,
    #[serde(with = "my_date_format")]
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

pub mod my_date_format {
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

pub struct Temperatures {
    pub temperatures: HashMap<NaiveDateTime, f32>,
}

use chrono::{DateTime, Utc};
impl Serialize for Temperatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.temperatures.len()))?;
        for (k, v) in &self.temperatures {
            let s = DateTime::<Utc>::from_utc(*k, Utc).to_rfc3339();
            map.serialize_entry(&s, &v)?;
        }
        map.end()
    }
}
