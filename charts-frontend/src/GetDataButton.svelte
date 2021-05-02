<script lang="ts">
  import { temperature } from "./stores";

  const handleClick = async () => {
    const response = await fetch("http://localhost:8000/db/data/temperature");
    const responseJson = await response.json();
    temperature.set(responseJson.temperature);
  };
</script>

<button on:click={handleClick}> Click me </button>
