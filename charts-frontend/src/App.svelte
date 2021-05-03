<script lang="ts">
  import Chart from "./Chart.svelte";
  import GetDataButton from "./GetDataButton.svelte";
  import { temperature, humidity, light, soilMoisture } from "./stores";

  $: data = {
    datasets: [
      {
        label: "Temperature",
        data: $temperature,
        fill: false,
        borderColor: "rgb(75, 192, 192)",
      },
      {
        label: "Humidity",
        data: $humidity,
        fill: false,
        borderColor: "rgb(0, 0, 192)",
      },
    ],
  };
  $: data2 = {
    datasets: [
      {
        label: "Light",
        data: $light,
        fill: false,
        borderColor: "rgb(0, 192, 0)",
      },
      {
        label: "Soil moisture",
        data: $soilMoisture,
        fill: false,
        borderColor: "rgb(192, 0, 0)",
      },
    ],
  };
</script>

<main>
  <div class="container-fluid">
    <div class="row">
      <div class="col-2 text-white bg-dark">A column</div>
      <div class="col-10">
        <div class="row">
          <div class="col-2" />
          <div class="col">
            <h1 class="py-3 text-center text-success">Plant Business</h1>
          </div>
          <div class="col-2 d-flex justify-content-center align-items-center">
            <GetDataButton />
          </div>
        </div>
        <Chart {data} />
        <Chart data={data2} />
      </div>
    </div>
  </div>
</main>
