<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { enGB } from "date-fns/locale";
  import { format } from "date-fns";
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
  import { pollForUpdates, relativeScale } from "./stores";

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

  let chartStartDate: string = "";
  let chartStartTime: string = "";
  let chartEndDate: string = "";
  let chartEndTime: string = "";

  $: chartMin =
    chartStartDate === ""
      ? undefined
      : +new Date(
          `${chartStartDate} ${
            chartStartTime === "" ? "00:00:00" : chartStartTime
          }`
        );
  $: chartMax =
    chartEndDate === ""
      ? undefined
      : +new Date(
          `${chartEndDate} ${chartEndTime === "" ? "23:59:59" : chartEndTime}`
        );

  $: updateChartScale(chartMin, chartMax);

  let timeoutId: number;
  const refreshData = () => {
    handleGetDataUpdate();
    timeoutId = window.setTimeout(refreshData, chartUpdateIntervalMs);
  };
  // Whenever the update interval changes, update the timer
  const resetDataTimer = (pollForUpdates: boolean, intervalMs: number) => {
    clearTimeout(timeoutId);
    if (pollForUpdates) {
      timeoutId = window.setTimeout(refreshData, intervalMs);
    }
  };
  $: resetDataTimer($pollForUpdates, chartUpdateIntervalMs);

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

  const updateChartScale = (
    chartMin: number | undefined,
    chartMax: number | undefined
  ) => {
    if (!chart) return;
    chart.options.scales.x.min = chartMin;
    chart.options.scales.x.max = chartMax;
    chart.update();
  };

  let canvasElement: HTMLCanvasElement;
  let chart: Chart<"line", TimeseriesData[]> = null;
  let maxTimestamp: string;
  onMount(async () => {
    const data = { datasets: getDatasets(await getInitialData()) };
    maxTimestamp = data.datasets
      .map((dataset) => dataset.data[dataset.data.length - 1]) // Get last data point for each data set
      .sort((a, b) => b.time.localeCompare(a.time))[0].time; // Get last data point across data sets

    const minTimestamp = data.datasets
      .map((dataset) => dataset.data[0])
      .sort((a, b) => a.time.localeCompare(b.time))[0].time;

    const chartStart = new Date(minTimestamp);
    chartStartDate = format(chartStart, "yyyy-MM-dd");
    chartStartTime = format(chartStart, "HH:mm:ss");
    const chartEnd = new Date(new Date(maxTimestamp).getTime() + 1000); // Add 1 second to ensure end included on chart
    chartEndDate = format(chartEnd, "yyyy-MM-dd");
    chartEndTime = format(chartEnd, "HH:mm:ss");

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

    if ($pollForUpdates) {
      refreshData();
    }
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
    <div class="input-group">
      <span class="input-group-text" id="chart-start-date"
        >Chart start date</span
      >
      <input
        type="date"
        class="form-control"
        aria-describedby="chart-start-date"
        bind:value={chartStartDate}
      />
      <span class="input-group-text" id="chart-start-time"
        >Chart start time</span
      >
      <input
        type="time"
        step="1"
        class="form-control"
        aria-describedby="chart-start-time"
        bind:value={chartStartTime}
      />
      <span class="input-group-text" id="chart-end-date">Chart end date</span>
      <input
        type="date"
        class="form-control"
        aria-describedby="chart-end-date"
        bind:value={chartEndDate}
      />
      <span class="input-group-text" id="chart-end-time">Chart end time</span>
      <input
        type="time"
        step="1"
        class="form-control"
        aria-describedby="chart-end-time"
        bind:value={chartEndTime}
      />
    </div>
  </form>
</div>
