<script lang="ts">
  import type Chart from "./Chart.svelte";

  export let chart: Chart;
</script>

<button
  type="button"
  class="btn btn-primary"
  on:click={() => {
    chart.handleGetDataUpdate();
  }}>Refresh</button
>
