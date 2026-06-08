<script lang="ts">
  import { selectedPlugin } from "../stores";
  import { togglePlugin, deletePlugin, revealInFinder } from "../commands";
  import { plugins, selectedPluginId, errorMessage } from "../stores";

  const formatLabels: Record<string, string> = {
    AudioUnit: "Audio Unit (AU)",
    Vst2: "VST2",
    Vst3: "VST3",
    Aax: "AAX",
    Clap: "CLAP",
  };

  function formatSize(bytes: number): string {
    if (bytes === 0) return "-";
    const units = ["B", "KB", "MB", "GB"];
    let i = 0;
    let size = bytes;
    while (size >= 1024 && i < units.length - 1) {
      size /= 1024;
      i++;
    }
    return `${size.toFixed(2)} ${units[i]}`;
  }

  async function handleToggle() {
    if (!$selectedPlugin) return;
    try {
      const enabled = await togglePlugin($selectedPlugin.id, !$selectedPlugin.enabled);
      $plugins = $plugins.map((p) =>
        p.id === $selectedPlugin!.id ? { ...p, enabled } : p
      );
    } catch (e) {
      console.error("Failed to toggle plugin", e);
    }
  }

  async function handleDelete() {
    if (!$selectedPlugin) return;
    const p = $selectedPlugin;
    const msg = `Permanently delete "${p.name}"?\n\nFormat: ${formatLabels[p.format] || p.format}\nPath: ${p.path}\nSize: ${formatSize(p.size_bytes)}\n\nThis will remove the plugin files from your system. This cannot be undone.`;
    if (!confirm(msg)) return;

    try {
      await deletePlugin(p.id);
      $plugins = $plugins.filter((pl) => pl.id !== p.id);
      $selectedPluginId = null;
    } catch (e: any) {
      $errorMessage = `Delete failed: ${e}`;
    }
  }

  async function handleReveal(path: string) {
    try {
      await revealInFinder(path);
    } catch (e) {
      console.error("Failed to reveal in Finder", e);
    }
  }
</script>

{#if $selectedPlugin}
  <div class="detail-panel">
    <div class="detail-header">
      <h2>{$selectedPlugin.name}</h2>
      <button class="btn-close" onclick={() => ($selectedPluginId = null)}>✕</button>
    </div>

    <div class="detail-body">
      <div class="field">
        <span class="field-label">Status</span>
        <div class="field-value">
          <span class="status-badge" class:enabled={$selectedPlugin.enabled} class:disabled={!$selectedPlugin.enabled}>
            {$selectedPlugin.enabled ? "Enabled" : "Disabled"}
          </span>
          <button class="btn btn-sm" onclick={handleToggle}>
            {$selectedPlugin.enabled ? "Disable" : "Enable"}
          </button>
        </div>
      </div>

      <div class="field">
        <span class="field-label">Format</span>
        <div class="field-value">{formatLabels[$selectedPlugin.format] || $selectedPlugin.format}</div>
      </div>

      <div class="field">
        <span class="field-label">Vendor</span>
        <div class="field-value">{$selectedPlugin.vendor || "-"}</div>
      </div>

      <div class="field">
        <span class="field-label">Version</span>
        <div class="field-value">{$selectedPlugin.version || "-"}</div>
      </div>

      <div class="field">
        <span class="field-label">Architecture</span>
        <div class="field-value">{$selectedPlugin.arch}</div>
      </div>

      <div class="field">
        <span class="field-label">Bundle ID</span>
        <div class="field-value mono">{$selectedPlugin.bundle_id}</div>
      </div>

      <div class="field">
        <span class="field-label">Size on Disk</span>
        <div class="field-value">{formatSize($selectedPlugin.size_bytes)}</div>
      </div>

      <div class="field">
        <span class="field-label">Path</span>
        <div class="field-value mono path">{($selectedPlugin.path.length > 50 ? "…" + $selectedPlugin.path.slice(-50) : $selectedPlugin.path)}</div>
      </div>

      <div class="actions">
        <button class="btn" onclick={() => handleReveal($selectedPlugin.path)}>
          Reveal in Finder
        </button>
        <button class="btn btn-danger" onclick={handleDelete}>
          Delete from Disk
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .detail-panel {
    width: 300px;
    border-left: 1px solid var(--border);
    background: var(--surface);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
  }

  .detail-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 16px;
    border-bottom: 1px solid var(--border);
  }

  .detail-header h2 {
    margin: 0;
    font-size: 15px;
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .btn-close {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 16px;
    color: var(--text-secondary);
    padding: 2px 6px;
    border-radius: 4px;
  }

  .btn-close:hover {
    background: var(--hover);
    color: var(--text);
  }

  .detail-body {
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .field .field-label {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--text-secondary);
  }

  .field-value {
    font-size: 13px;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .mono {
    font-family: monospace;
    font-size: 11px;
    word-break: break-all;
  }

  .path {
    color: var(--text-secondary);
  }

  .status-badge {
    display: inline-block;
    padding: 2px 8px;
    border-radius: 10px;
    font-size: 11px;
    font-weight: 600;
  }

  .status-badge.enabled {
    background: #065f46;
    color: #6ee7b7;
  }

  .status-badge.disabled {
    background: #374151;
    color: #9ca3af;
  }

  .btn {
    padding: 6px 14px;
    border: 1px solid var(--border);
    border-radius: 6px;
    font-size: 12px;
    cursor: pointer;
    background: var(--bg);
    color: var(--text);
    transition: background 0.15s;
  }

  .btn:hover {
    background: var(--hover);
  }

  .btn-sm {
    padding: 3px 10px;
    font-size: 11px;
  }

  .btn-danger {
    color: #ef4444;
    border-color: #ef4444;
  }

  .btn-danger:hover {
    background: rgba(239, 68, 68, 0.1);
  }

  .actions {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-top: 8px;
    padding-top: 12px;
    border-top: 1px solid var(--border);
  }
</style>
