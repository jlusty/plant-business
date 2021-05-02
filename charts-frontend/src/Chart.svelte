<script lang="ts">
  import { onMount } from "svelte";
  import {
    Chart,
    LineElement,
    LineController,
    CategoryScale,
    LinearScale,
    PointElement,
    Tooltip,
  } from "chart.js";

  Chart.register(
    LineElement,
    LineController,
    CategoryScale,
    LinearScale,
    PointElement,
    Tooltip
  );
  Chart.overrides.line.spanGaps = true;

  let canvasElement: HTMLCanvasElement;

  onMount(() => {
    new Chart(canvasElement, {
      type: "line",
      data: {
        labels: [0, 1, 2, 3, 4, 5, 6, 7],
        datasets: [
          {
            label: "Temperature",
            data: [65, 59, 80, 81, 31, 85, 40],
            fill: false,
            borderColor: "rgb(75, 192, 192)",
          },
          {
            label: "Humidity",
            data: [65, 59, 30, 81, null, 55, 40],
            fill: false,
            borderColor: "rgb(0, 0, 192)",
          },
        ],
      },
    });
  });
</script>

<canvas bind:this={canvasElement} width={100} height={30} />
