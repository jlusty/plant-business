<script lang="ts">
  import { beforeUpdate, onDestroy, onMount } from "svelte";
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
  import { getInitialData, getUpdateData } from "./refreshData";
  import type { TimeseriesData } from "./refreshData";
  import { relativeScale } from "./stores";

  let chartUpdateIntervalSecondsAllowedRange = [1, 86400];
  let chartUpdateIntervalSeconds = 10;
  $: chartUpdateIntervalMs =
    Math.min(
      Math.max(
        chartUpdateIntervalSecondsAllowedRange[0],
        chartUpdateIntervalSeconds
      ),
      chartUpdateIntervalSecondsAllowedRange[1]
    ) * 1000;

  let timeoutId: number;
  const refreshData = () => {
    handleGetDataUpdate();
    timeoutId = setTimeout(refreshData, chartUpdateIntervalMs);
  };
  // Whenever the update interval changes, update the timer
  const resetDataTimer = (intervalMs: number) => {
    clearTimeout(timeoutId);
    timeoutId = setTimeout(refreshData, intervalMs);
  };
  $: resetDataTimer(chartUpdateIntervalMs);

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

  const getDatasets = (data: Record<string, TimeseriesData[]>) => [
    {
      label: "Temperature",
      data: data.temperature,
      fill: false,
      borderColor: "rgb(75, 192, 192)",
    },
    {
      label: "Humidity",
      data: data.humidity,
      fill: false,
      borderColor: "rgb(0, 0, 192)",
    },
    {
      label: "Light",
      data: data.light,
      fill: false,
      borderColor: "rgb(0, 192, 0)",
    },
    {
      label: "Soil moisture",
      data: data.soilMoisture,
      fill: false,
      borderColor: "rgb(192, 0, 0)",
    },
  ];

  const addRelativeData = (data: TimeseriesData[]) => {
    const dataPoints: number[] = data.map(({ data }) => data);
    const maxValue = Math.max(...dataPoints);
    const minValue = Math.min(...dataPoints);
    const divisor = maxValue === minValue ? 1 : maxValue - minValue;
    for (let dataPoint of data) {
      dataPoint.relativeData = (dataPoint.data - minValue) / divisor;
    }
  };

  $: updateChartData($relativeScale);

  const updateChartData = (relativeScale: boolean) => {
    if (!chart) return;
    if (!relativeScale) {
      if (chart.options.parsing["yAxisKey"] !== "data") {
        chart.options.parsing["yAxisKey"] = "data";
        chart.update();
      }
    } else {
      for (const dataset of chart.data.datasets) {
        addRelativeData(dataset.data);
      }
      if (chart.options.parsing["yAxisKey"] !== "relativeData") {
        chart.options.parsing["yAxisKey"] = "relativeData";
        chart.update();
      }
    }
  };

  let canvasElement: HTMLCanvasElement;
  let chart: Chart<"line", TimeseriesData[]> = null;
  let maxTimestamp: string;
  onMount(async () => {
    const data = { datasets: getDatasets(await getInitialData()) };
    maxTimestamp = data.datasets
      .map((dataset) => dataset.data[dataset.data.length - 1])
      .sort((a, b) => b.time.localeCompare(a.time))[0].time;

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
              displayFormats: {
                second: "ss",
                minute: "HH:mm",
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

    refreshData();
  });

  onDestroy(() => {
    chart = null;
    clearTimeout(timeoutId);
  });

  export const handleGetDataUpdate = async () => {
    const dataUpdate = await getUpdateData(maxTimestamp);
    for (const dataset of chart.data.datasets) {
      dataset.data.push(...dataUpdate[dataset.label]);
    }
    const maxData = chart.data.datasets
      .map((dataset) => dataset.data[dataset.data.length - 1])
      .sort((a, b) => b.time.localeCompare(a.time))[0];
    maxTimestamp = maxData.time;
    chart.update();
  };
</script>

<div class="py-4 px-3">
  <canvas bind:this={canvasElement} width={100} height={40} />
  <form>
    <div class="input-group">
      <span class="input-group-text" id="update-frequency"
        >Update frequency (seconds)</span
      >
      <input
        type="number"
        class="form-control"
        aria-describedby="update-frequency"
        min={chartUpdateIntervalSecondsAllowedRange[0]}
        max={chartUpdateIntervalSecondsAllowedRange[1]}
        bind:value={chartUpdateIntervalSeconds}
      />
    </div>
  </form>
</div>
