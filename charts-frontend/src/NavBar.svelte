<script lang="ts">
  import { pollForUpdates, relativeScale } from "./stores";
</script>

<div class="list-group-flush py-4">
  <label class="list-group-item text-white bg-dark">
    <input
      class="form-check-input me-1"
      type="checkbox"
      value=""
      bind:checked={$pollForUpdates}
    />
    Poll for updates
  </label>
  <hr />
  <div class="form-check form-switch">
    <input
      class="form-check-input"
      type="checkbox"
      id="relativeScaleSwitch"
      bind:checked={$relativeScale}
    />
    <label class="form-check-label" for="relativeScaleSwitch"
      >Relative scale</label
    >
  </div>
</div>
