<script lang="ts">
  import { getInitialData } from "./refreshData";
</script>

<button type="button" class="btn btn-primary" on:click={getInitialData}
  >Refresh</button
>
