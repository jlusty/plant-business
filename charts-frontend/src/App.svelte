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
      <div class="col-2 text-white bg-dark sidebar">
        <div class="position-sticky">
          <div class="list-group-flush py-4">
            <label class="list-group-item text-white bg-dark">
              <input class="form-check-input me-1 " type="checkbox" value="" />
              First checkbox
            </label>
            <label class="list-group-item text-white bg-dark">
              <input class="form-check-input me-1" type="checkbox" value="" />
              Second checkbox
            </label>
            <label class="list-group-item text-white bg-dark">
              <input class="form-check-input me-1" type="checkbox" value="" />
              Third checkbox
            </label>
            <label class="list-group-item text-white bg-dark">
              <input class="form-check-input me-1" type="checkbox" value="" />
              Fourth checkbox
            </label>
          </div>
        </div>
      </div>
      <div class="col-10 px-4 ms-auto">
        <div class="row border-bottom">
          <div class="col-2" />
          <div class="col">
            <h1 class="py-3 display-2 text-center text-success">
              Plant Business
            </h1>
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
