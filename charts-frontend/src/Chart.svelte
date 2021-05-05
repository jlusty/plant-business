<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { enGB } from "date-fns/locale";
  import "chartjs-adapter-date-fns";
  import {
    Chart,
    LineElement,
    LineController,
    LinearScale,
    TimeScale,
    PointElement,
    Tooltip,
    Legend,
  } from "chart.js";
  import { temperature, humidity, light, soilMoisture } from "./stores";
  import { getInitialData, getUpdateData } from "./refreshData";

  const CHART_UPDATE_INTERVAL_MS = 10000;

  Chart.register(
    LineElement,
    LineController,
    LinearScale,
    TimeScale,
    PointElement,
    Tooltip,
    Legend
  );
  Chart.overrides.line.spanGaps = true;

  const getDatasets = (data: Record<string, any>) => [
    {
      label: "Temperature",
      name: "temperature",
      data: data.temperature,
      isVisible: $temperature.isVisible,
      fill: false,
      borderColor: "rgb(75, 192, 192)",
    },
    {
      label: "Humidity",
      name: "humidity",
      data: data.humidity,
      isVisible: $humidity.isVisible,
      fill: false,
      borderColor: "rgb(0, 0, 192)",
    },
    {
      label: "Light",
      name: "light",
      data: data.light,
      isVisible: $light.isVisible,
      fill: false,
      borderColor: "rgb(0, 192, 0)",
    },
    {
      label: "Soil moisture",
      name: "soilMoisture",
      data: data.soilMoisture,
      isVisible: $soilMoisture.isVisible,
      fill: false,
      borderColor: "rgb(192, 0, 0)",
    },
  ];

  // $: data = {
  //   datasets: datasets
  //     .filter((d) => d.isVisible)
  //     .map(({ isVisible, ...d }) => {
  //       if ($relativeScale) {
  //         const dataPoints: number[] = d.data.map(({ data }) => data);
  //         const maxValue = Math.max(...dataPoints);
  //         const minValue = Math.min(...dataPoints);
  //         const divisor = maxValue === minValue ? 1 : maxValue - minValue;
  //         d.data = d.data.map(({ time, data }) => ({
  //           time,
  //           data: (data - minValue) / divisor,
  //         }));
  //       }
  //       return d;
  //     }),
  // };

  let canvasElement: HTMLCanvasElement;
  let chart: Chart = null;
  let maxTimestamp: string;
  onMount(async () => {
    const data = { datasets: getDatasets(await getInitialData()) };
    maxTimestamp = data.datasets
      .map((dataset) => dataset.data[dataset.data.length - 1])
      .sort((a, b) => b.time.localeCompare(a.time))[0].time;
    console.log(data);
    console.log(maxTimestamp);
    chart = new Chart(canvasElement, {
      type: "line",
      data,
      options: {
        parsing: {
          xAxisKey: "time",
          yAxisKey: "data",
        },
        scales: {
          x: {
            type: "time",
            adapters: { date: { locale: enGB } },
            time: {
              unit: "hour",
              displayFormats: {
                hour: "HH:mm",
              },
            },
            ticks: { major: { enabled: true } },
          },
        },
        plugins: {
          legend: {
            display: true,
            position: "right",
          },
        },
      },
    });

    setInterval(() => {
      handleGetDataUpdate();
    }, CHART_UPDATE_INTERVAL_MS);
  });

  onDestroy(() => {
    chart = null;
  });

  const handleGetDataUpdate = async () => {
    const dataUpdate = await getUpdateData(maxTimestamp);
    for (const dataset of chart.data.datasets) {
      dataset.data.push(...dataUpdate[(<any>dataset).name]);
    }
    const maxData: any = chart.data.datasets
      .map((dataset) => dataset.data[dataset.data.length - 1])
      .sort((a, b) => (<any>b).time.localeCompare((<any>a).time))[0];
    maxTimestamp = maxData.time;
    console.log(maxTimestamp);
    chart.update();
  };
</script>

<div class="py-4 px-3">
  <canvas bind:this={canvasElement} width={100} height={40} />
</div>
<button on:click={handleGetDataUpdate}>Update data</button>
