table! {
    plant_metrics (recorded_at) {
        recorded_at -> Timestamp,
        temperature -> Nullable<Float4>,
        humidity -> Nullable<Float4>,
        light -> Nullable<Int4>,
        soil_moisture -> Nullable<Int4>,
    }
}
