CREATE TABLE plant_metrics (
  recorded_at TIMESTAMP PRIMARY KEY,
  temperature REAL,
  humidity REAL,
  light INTEGER,
  soil_moisture INTEGER
)