<script lang="ts">
  import { onMount } from "svelte";
  import Toolbar from "./lib/components/Toolbar.svelte";
  import PluginTable from "./lib/components/PluginTable.svelte";
  import PluginDetail from "./lib/components/PluginDetail.svelte";
  import { plugins, errorMessage } from "./lib/stores";
  import { getPlugins } from "./lib/commands";

  onMount(async () => {
    try {
      const result = await getPlugins();
      $plugins = result;
    } catch (e) {
      console.error("Failed to load plugins", e);
    }
  });
</script>

<div class="app">
  {#if $errorMessage}
    <button class="error-banner" onclick={() => ($errorMessage = null)}>
      <span class="error-text">{$errorMessage}</span>
      <span class="error-dismiss">✕</span>
    </button>
  {/if}
  <header class="app-header">
    <h1>PluginVault</h1>
    <span class="subtitle">Audio Plugin Manager</span>
  </header>
  <Toolbar />
  <main class="main-content">
    <PluginTable />
    <PluginDetail />
  </main>
</div>

<style>
  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  :global(:root) {
    --bg: #0f1117;
    --surface: #1a1b23;
    --hover: #252630;
    --selected: #1e2030;
    --border: #2a2b35;
    --text: #e1e2e8;
    --text-secondary: #8b8d9a;
    --accent: #6366f1;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    font-size: 14px;
    color: var(--text);
    background: var(--bg);
  }

  :global(body) {
    margin: 0;
    overflow: hidden;
  }

  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }

  .error-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 16px;
    background: #7f1d1d;
    color: #fca5a5;
    font-size: 13px;
    cursor: pointer;
    border-bottom: 1px solid #991b1b;
  }

  .error-text {
    flex: 1;
  }

  .error-dismiss {
    margin-left: 12px;
    font-size: 14px;
    opacity: 0.7;
  }

  .app-header {
    display: flex;
    align-items: baseline;
    gap: 10px;
    padding: 12px 16px 0;
  }

  .app-header h1 {
    font-size: 18px;
    font-weight: 700;
    background: linear-gradient(135deg, #6366f1, #a855f7);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .subtitle {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .main-content {
    flex: 1;
    display: flex;
    overflow: hidden;
  }
</style>
