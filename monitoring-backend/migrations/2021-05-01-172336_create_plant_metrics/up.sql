CREATE TABLE plant_metrics (
  id SERIAL PRIMARY KEY,
  recorded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  temperature REAL,
  humidity REAL,
  light INTEGER,
  soil_moisture INTEGER
)