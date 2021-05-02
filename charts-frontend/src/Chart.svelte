<script lang="ts">
  import { onMount } from "svelte";
  // import {
  //   Chart,
  //   LineElement,
  //   LineController,
  //   CategoryScale,
  //   LinearScale,
  //   TimeScale,
  //   PointElement,
  //   Tooltip,
  // } from "chart.js";

  // Chart.register(
  //   LineElement,
  //   LineController,
  //   CategoryScale,
  //   LinearScale,
  //   TimeScale,
  //   PointElement,
  //   Tooltip
  // );
  import Chart from "chart.js/auto";

  Chart.overrides.line.spanGaps = true;

  let canvasElement: HTMLCanvasElement;

  const tempData = {
    temperature: [
      {
        time: "2021-05-02T13:08:26.684215+00:00",
        data: 0.1,
      },
      {
        time: "2021-05-02T01:06:30.774057+00:00",
        data: 0.1,
      },
      {
        time: "2021-05-01T23:46:29.589740+00:00",
        data: 0.1,
      },
      {
        time: "2021-05-01T23:45:23.835608+00:00",
        data: 0.1,
      },
      {
        time: "2021-05-01T23:35:51.283+00:00",
        data: 0.1,
      },
      {
        time: "2021-05-01T23:34:33.953324+00:00",
        data: 0.1,
      },
      {
        time: "2021-05-01T23:34:31.952755+00:00",
        data: 35,
      },
      {
        time: "2021-05-01T23:30:49.669813+00:00",
        data: 0.1,
      },
      {
        time: "2021-05-01T23:30:48.215167+00:00",
        data: 0.1,
      },
      {
        time: "2021-05-01T23:30:46.877377+00:00",
        data: 0.1,
      },
      {
        time: "2021-05-01T23:30:43.490076+00:00",
        data: 0.1,
      },
    ],
  };

  const humidData = {
    humidity: [
      {
        time: "2021-05-02T06:30:46.877377+00:00",
        data: 25,
      },
      {
        time: "2021-05-01T23:30:43.490076+00:00",
        data: 0.1,
      },
    ],
  };

  import { enGB } from "date-fns/locale";
  import "chartjs-adapter-date-fns";

  onMount(() => {
    new Chart(canvasElement, {
      type: "line",
      data: {
        datasets: [
          {
            label: "Temperature",
            data: tempData.temperature,
            fill: false,
            borderColor: "rgb(75, 192, 192)",
          },
          {
            label: "Humidity",
            data: humidData.humidity,
            fill: false,
            borderColor: "rgb(0, 0, 192)",
          },
        ],
      },
      options: {
        parsing: {
          xAxisKey: "time",
          yAxisKey: "data",
        },
        scales: {
          x: {
            type: "time",
            adapters: { date: { locale: enGB } },
            // time: {
            //   displayFormats: {
            //     quarter: "MMM YYYY",
            //   },
            // },
          },
        },
      },
    });
  });
</script>

<canvas bind:this={canvasElement} width={100} height={30} />
