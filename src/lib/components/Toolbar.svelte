<script lang="ts">
  import { searchQuery, formatFilter, vendorFilter, vendors, isScanning } from "../stores";
  import { scanPlugins, getPlugins } from "../commands";
  import { plugins } from "../stores";

  async function handleScan() {
    $isScanning = true;
    try {
      await scanPlugins();
      const result = await getPlugins();
      $plugins = result;
    } finally {
      $isScanning = false;
    }
  }

  async function handleRefresh() {
    try {
      const result = await getPlugins();
      $plugins = result;
    } catch (e) {
      console.error("Failed to load plugins", e);
    }
  }
</script>

<div class="toolbar">
  <div class="toolbar-left">
    <button class="btn btn-primary" onclick={handleScan} disabled={$isScanning}>
      {#if $isScanning}
        Scanning…
      {:else}
        Scan Plugins
      {/if}
    </button>
    <button class="btn" onclick={handleRefresh} disabled={$isScanning}>
      Refresh
    </button>
  </div>
  <div class="toolbar-center">
    <input
      type="search"
      placeholder="Search plugins…"
      bind:value={$searchQuery}
      class="search-input"
    />
  </div>
  <div class="toolbar-right">
    <select bind:value={$formatFilter} class="filter-select">
      <option value="all">All Formats</option>
      <option value="AudioUnit">AU</option>
      <option value="Vst3">VST3</option>
    </select>
    <select bind:value={$vendorFilter} class="filter-select">
      <option value="all">All Vendors</option>
      {#each $vendors as v}
        <option value={v}>{v}</option>
      {/each}
    </select>
  </div>
</div>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    background: var(--surface);
    border-bottom: 1px solid var(--border);
  }

  .toolbar-left {
    display: flex;
    gap: 8px;
  }

  .toolbar-center {
    flex: 1;
  }

  .toolbar-right {
    display: flex;
    gap: 8px;
  }

  .search-input {
    width: 100%;
    padding: 6px 12px;
    border: 1px solid var(--border);
    border-radius: 6px;
    font-size: 13px;
    background: var(--bg);
    color: var(--text);
    outline: none;
  }

  .search-input:focus {
    border-color: var(--accent);
  }

  .filter-select {
    padding: 6px 10px;
    border: 1px solid var(--border);
    border-radius: 6px;
    font-size: 13px;
    background: var(--bg);
    color: var(--text);
    cursor: pointer;
  }

  .btn {
    padding: 6px 14px;
    border: 1px solid var(--border);
    border-radius: 6px;
    font-size: 13px;
    cursor: pointer;
    background: var(--surface);
    color: var(--text);
    transition: background 0.15s;
  }

  .btn:hover {
    background: var(--hover);
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-primary {
    background: var(--accent);
    color: #fff;
    border-color: var(--accent);
  }

  .btn-primary:hover {
    filter: brightness(1.1);
  }
</style>
