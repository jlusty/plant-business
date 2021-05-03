<script lang="ts">
  import { temperature, humidity, light, soilMoisture } from "./stores";
</script>

<div class="list-group-flush py-4">
  <label class="list-group-item text-white bg-dark">
    <input
      class="form-check-input me-1"
      type="checkbox"
      value=""
      bind:checked={$temperature.isVisible}
    />
    Temperature
  </label>
  <label class="list-group-item text-white bg-dark">
    <input
      class="form-check-input me-1"
      type="checkbox"
      value=""
      bind:checked={$humidity.isVisible}
    />
    Humidity
  </label>
  <label class="list-group-item text-white bg-dark">
    <input
      class="form-check-input me-1"
      type="checkbox"
      value=""
      bind:checked={$light.isVisible}
    />
    Light
  </label>
  <label class="list-group-item text-white bg-dark">
    <input
      class="form-check-input me-1"
      type="checkbox"
      value=""
      bind:checked={$soilMoisture.isVisible}
    />
    Soil Moisture
  </label>
</div>
