table! {
    plant_metrics (id) {
        id -> Int4,
        recorded_at -> Timestamp,
        temperature -> Nullable<Float4>,
        humidity -> Nullable<Float4>,
        light -> Nullable<Int4>,
        soil_moisture -> Nullable<Int4>,
    }
}
