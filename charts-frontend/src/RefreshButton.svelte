<script lang="ts">
  export let chart;
</script>

<button
  type="button"
  class="btn btn-primary"
  on:click={chart.handleGetDataUpdate()}>Refresh</button
>
