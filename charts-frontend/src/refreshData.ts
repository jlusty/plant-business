import { DATA_SERVER_IP } from "./constants";
import { temperature, humidity, light, soilMoisture } from "./stores";

export const refreshData = () => {
  fetch(`http://${DATA_SERVER_IP}/db/data/temperature`)
    .then((res) => res.json())
    .then((res) =>
      temperature.update(({ isVisible }) => ({
        isVisible,
        data: res.temperature,
      }))
    );
  fetch(`http://${DATA_SERVER_IP}/db/data/humidity`)
    .then((res) => res.json())
    .then((res) =>
      humidity.update(({ isVisible }) => ({
        isVisible,
        data: res.humidity,
      }))
    );
  fetch(`http://${DATA_SERVER_IP}/db/data/light`)
    .then((res) => res.json())
    .then((res) =>
      light.update(({ isVisible }) => ({
        isVisible,
        data: res.light,
      }))
    );
  fetch(`http://${DATA_SERVER_IP}/db/data/soilmoisture`)
    .then((res) => res.json())
    .then((res) =>
      soilMoisture.update(({ isVisible }) => ({
        isVisible,
        data: res.soil_moisture,
      }))
    );
};
