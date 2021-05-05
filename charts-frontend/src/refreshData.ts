import { DATA_SERVER_IP } from "./constants";

export const getInitialData = async () => {
  const data: Record<string, any> = {};
  await Promise.all([
    fetch(`http://${DATA_SERVER_IP}/db/data/temperature`)
      .then((res) => res.json())
      .then((res) => {
        data.temperature = res.temperature;
      }),
    fetch(`http://${DATA_SERVER_IP}/db/data/humidity`)
      .then((res) => res.json())
      .then((res) => {
        data.humidity = res.humidity;
      }),
    fetch(`http://${DATA_SERVER_IP}/db/data/light`)
      .then((res) => res.json())
      .then((res) => {
        data.light = res.light;
      }),
    fetch(`http://${DATA_SERVER_IP}/db/data/soilmoisture`)
      .then((res) => res.json())
      .then((res) => {
        data.soilMoisture = res.soil_moisture;
      }),
  ]);
  return data;
};

export const getUpdateData = async (time: string) => {
  const data: Record<string, any> = {};
  await Promise.all([
    fetch(`http://${DATA_SERVER_IP}/db/data/temperature/${time}`)
      .then((res) => res.json())
      .then((res) => {
        data.temperature = res.temperature;
      }),
    fetch(`http://${DATA_SERVER_IP}/db/data/humidity/${time}`)
      .then((res) => res.json())
      .then((res) => {
        data.humidity = res.humidity;
      }),
    fetch(`http://${DATA_SERVER_IP}/db/data/light/${time}`)
      .then((res) => res.json())
      .then((res) => {
        data.light = res.light;
      }),
    fetch(`http://${DATA_SERVER_IP}/db/data/soilmoisture/${time}`)
      .then((res) => res.json())
      .then((res) => {
        data.soilMoisture = res.soil_moisture;
      }),
  ]);
  return data;
};
