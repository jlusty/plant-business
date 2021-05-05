<script lang="ts">
  import NavBar from "./NavBar.svelte";
  import Chart from "./Chart.svelte";
  import GetDataButton from "./RefreshButton.svelte";

  export let chart;
</script>

<main>
  <div class="container-fluid">
    <div class="row">
      <div class="col-2 text-white bg-dark sidebar">
        <div class="position-sticky">
          <NavBar />
        </div>
      </div>
      <div class="col-10 px-4 ms-auto">
        <div class="row border-bottom">
          <div class="col-2" />
          <div class="col">
            <h1 class="py-3 display-2 text-center text-success">
              Plant Business
            </h1>
          </div>
          <div class="col-2 d-flex justify-content-center align-items-center">
            <GetDataButton {chart} />
          </div>
        </div>
        <Chart bind:this={chart} />
      </div>
    </div>
  </div>
</main>

<style>
  .sidebar {
    position: fixed;
    top: 0;
    bottom: 0;
    left: 0;
  }
</style>
