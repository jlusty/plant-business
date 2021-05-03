<script lang="ts">
  import NavBar from "./NavBar.svelte";
  import Chart from "./Chart.svelte";
  import GetDataButton from "./RefreshButton.svelte";
  import {
    temperature,
    humidity,
    light,
    soilMoisture,
    relativeScale,
  } from "./stores";
  import { onMount } from "svelte";
  import { refreshData } from "./refreshData";

  $: datasets = [
    {
      label: "Temperature",
      data: $temperature.data,
      isVisible: $temperature.isVisible,
      fill: false,
      borderColor: "rgb(75, 192, 192)",
    },
    {
      label: "Humidity",
      data: $humidity.data,
      isVisible: $humidity.isVisible,
      fill: false,
      borderColor: "rgb(0, 0, 192)",
    },
    {
      label: "Light",
      data: $light.data,
      isVisible: $light.isVisible,
      fill: false,
      borderColor: "rgb(0, 192, 0)",
    },
    {
      label: "Soil moisture",
      data: $soilMoisture.data,
      isVisible: $soilMoisture.isVisible,
      fill: false,
      borderColor: "rgb(192, 0, 0)",
    },
  ];

  $: maxValue = Math.max(
    ...datasets.flatMap((d) => {
      return d.data.map((entry) => entry.data);
    })
  );

  $: data = {
    datasets: datasets
      .filter((d) => d.isVisible)
      .map(({ isVisible, ...d }) => {
        if ($relativeScale) {
          d.data = d.data.map(({ time, data }) => ({
            time,
            data: data / maxValue,
          }));
        }
        console.log(`d is ${JSON.stringify(d)}`);
        return d;
      }),
  };

  onMount(() => refreshData());
</script>

<main>
  <div class="container-fluid">
    <div class="row">
      <div class="col-2 text-white bg-dark sidebar">
        <div class="position-sticky">
          <NavBar />
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
      </div>
    </div>
  </div>
</main>

<style>
  .sidebar {
    position: fixed;
    top: 0;
    bottom: 0;
    left: 0;
  }
</style>
