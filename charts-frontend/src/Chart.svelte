<script lang="ts">
  import { afterUpdate, onDestroy, onMount } from "svelte";
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
  } from "chart.js";
  import type { ChartData } from "chart.js";

  Chart.register(
    LineElement,
    LineController,
    LinearScale,
    TimeScale,
    PointElement,
    Tooltip
  );
  Chart.overrides.line.spanGaps = true;

  export let data: ChartData;

  let canvasElement: HTMLCanvasElement;
  let chart: Chart = null;
  onMount(() => {
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
      },
    });
  });
  afterUpdate(() => {
    if (!chart) return;

    chart.data = data;
    chart.update();
  });

  onDestroy(() => {
    chart = null;
  });
</script>

<div class="py-2 px-5">
  <canvas bind:this={canvasElement} width={100} height={50} />
</div>
