<script lang="ts">
  import { temperature, humidity, light, soilMoisture } from "./stores";

  const handleClick = () => {
    fetch("http://localhost:8000/db/data/temperature")
      .then((res) => res.json())
      .then((res) =>
        temperature.update(({ isVisible }) => ({
          isVisible,
          data: res.temperature,
        }))
      );
    fetch("http://localhost:8000/db/data/humidity")
      .then((res) => res.json())
      .then((res) =>
        humidity.update(({ isVisible }) => ({
          isVisible,
          data: res.humidity,
        }))
      );
    fetch("http://localhost:8000/db/data/light")
      .then((res) => res.json())
      .then((res) =>
        light.update(({ isVisible }) => ({
          isVisible,
          data: res.light,
        }))
      );
    fetch("http://localhost:8000/db/data/soilmoisture")
      .then((res) => res.json())
      .then((res) =>
        soilMoisture.update(({ isVisible }) => ({
          isVisible,
          data: res.soil_moisture,
        }))
      );
  };
</script>

<button type="button" class="btn btn-primary" on:click={handleClick}
  >Get Data</button
>
