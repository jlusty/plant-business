<script lang="ts">
  import { temperature, humidity, light, soilMoisture } from "./stores";

  const handleClick = () => {
    fetch("http://localhost:8000/db/data/temperature")
      .then((res) => res.json())
      .then((res) => temperature.set(res.temperature));
    fetch("http://localhost:8000/db/data/humidity")
      .then((res) => res.json())
      .then((res) => humidity.set(res.light));
    fetch("http://localhost:8000/db/data/temperature")
      .then((res) => res.json())
      .then((res) => light.set(res.light));
    fetch("http://localhost:8000/db/data/soilmoisture")
      .then((res) => res.json())
      .then((res) => soilMoisture.set(res.soil_moisture));
  };
</script>

<button on:click={handleClick}>Get Data</button>
