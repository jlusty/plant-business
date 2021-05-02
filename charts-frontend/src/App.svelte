<script lang="ts">
  import Chart from "./Chart.svelte";
  import GetDataButton from "./GetDataButton.svelte";
  import { temperature } from "./stores";

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
        data: [],
        fill: false,
        borderColor: "rgb(0, 0, 192)",
      },
    ],
  };
</script>

<main>
  <h1>Plant Business</h1>
  <GetDataButton />
  <Chart {data} />
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #13880f;
    font-size: 3em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
