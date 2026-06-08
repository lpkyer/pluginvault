<script lang="ts">
  import type { Plugin, SortField, SortDir } from "../types";
import { filteredPlugins, selectedPluginId, sortField, sortDir, plugins, errorMessage } from "../stores";
import { togglePlugin, deletePlugin } from "../commands";

  const formatLabels: Record<string, string> = {
    AudioUnit: "AU",
    Vst3: "VST3",
  };

  const archIcons: Record<string, string> = {
    AppleSilicon: "",
    Intel: "󰘓",
    Universal: "󰛓",
    Unknown: "?",
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
    return `${size.toFixed(1)} ${units[i]}`;
  }

  function handleSort(field: SortField) {
    if ($sortField === field) {
      $sortDir = $sortDir === "asc" ? "desc" : "asc";
    } else {
      $sortField = field;
      $sortDir = "asc";
    }
  }

  function sortArrow(field: SortField): string {
    if ($sortField !== field) return "";
    return $sortDir === "asc" ? " ▲" : " ▼";
  }

  async function handleToggle(plugin: Plugin) {
    try {
      const enabled = await togglePlugin(plugin.id, !plugin.enabled);
      $plugins = $plugins.map((p) =>
        p.id === plugin.id ? { ...p, enabled } : p
      );
    } catch (e) {
      console.error("Failed to toggle plugin", e);
    }
  }

  async function handleDelete(plugin: Plugin) {
    const msg = `Delete "${plugin.name}" (${plugin.format === "AudioUnit" ? "AU" : "VST3"})?\n\nPath: ${plugin.path}\n\nThis cannot be undone.`;
    if (!confirm(msg)) return;

    try {
      await deletePlugin(plugin.id);
      $plugins = $plugins.filter((p) => p.id !== plugin.id);
      if ($selectedPluginId === plugin.id) {
        $selectedPluginId = null;
      }
    } catch (e: any) {
      $errorMessage = `Delete failed: ${e}`;
    }
  }
</script>

<div class="table-container">
  <table class="plugin-table">
    <thead>
      <tr>
        <th class="col-status"></th>
        <th class="col-name sortable" onclick={() => handleSort("name")}>
          Name{sortArrow("name")}
        </th>
        <th class="col-vendor sortable" onclick={() => handleSort("vendor")}>
          Vendor{sortArrow("vendor")}
        </th>
        <th class="col-format sortable" onclick={() => handleSort("format")}>
          Format{sortArrow("format")}
        </th>
        <th class="col-version sortable" onclick={() => handleSort("version")}>
          Version{sortArrow("version")}
        </th>
        <th class="col-arch">Arch</th>
        <th class="col-size sortable" onclick={() => handleSort("size_bytes")}>
          Size{sortArrow("size_bytes")}
        </th>
        <th class="col-actions"></th>
      </tr>
    </thead>
    <tbody>
      {#each $filteredPlugins as plugin (plugin.id)}
        <tr
          class="plugin-row"
          class:selected={$selectedPluginId === plugin.id}
          onclick={() => ($selectedPluginId = plugin.id)}
        >
          <td class="col-status">
            <button
              class="toggle"
              class:enabled={plugin.enabled}
              class:disabled={!plugin.enabled}
              onclick={(e) => {
                e.stopPropagation();
                handleToggle(plugin);
              }}
              title={plugin.enabled ? "Disable" : "Enable"}
            >
              <span class="toggle-dot"></span>
            </button>
          </td>
          <td class="col-name">{plugin.name}</td>
          <td class="col-vendor">{plugin.vendor}</td>
          <td class="col-format">
            <span class="badge badge-{plugin.format.toLowerCase()}">
              {formatLabels[plugin.format] || plugin.format}
            </span>
          </td>
          <td class="col-version">{plugin.version || "-"}</td>
          <td class="col-arch">
            <span
              class="arch-badge"
              class:arm={plugin.arch === "AppleSilicon"}
              class:intel={plugin.arch === "Intel"}
              class:universal={plugin.arch === "Universal"}
              class:unknown={plugin.arch === "Unknown"}
              title={plugin.arch}
            >
              {plugin.arch === "AppleSilicon"
                ? "ARM"
                : plugin.arch === "Intel"
                  ? "x86"
                  : plugin.arch === "Universal"
                    ? "Uni"
                    : "?"}
            </span>
          </td>
          <td class="col-size">{formatSize(plugin.size_bytes)}</td>
          <td class="col-actions">
            <button
              class="btn-delete"
              title="Delete from disk"
              onclick={(e) => {
                e.stopPropagation();
                handleDelete(plugin);
              }}
            >
              ⌫
            </button>
          </td>
        </tr>
      {:else}
        <tr>
          <td colspan="8" class="empty-state">
            No plugins found. Click "Scan Plugins" to get started.
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .table-container {
    flex: 1;
    overflow-y: auto;
  }

  .plugin-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 13px;
  }

  thead {
    position: sticky;
    top: 0;
    background: var(--surface);
    z-index: 1;
  }

  th {
    text-align: left;
    padding: 8px 10px;
    font-weight: 600;
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--text-secondary);
    border-bottom: 1px solid var(--border);
    white-space: nowrap;
    user-select: none;
  }

  th.sortable {
    cursor: pointer;
  }

  th.sortable:hover {
    color: var(--text);
  }

  td {
    padding: 8px 10px;
    border-bottom: 1px solid var(--border);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .plugin-row {
    cursor: pointer;
    transition: background 0.1s;
  }

  .plugin-row:hover {
    background: var(--hover);
  }

  .plugin-row.selected {
    background: var(--selected);
  }

  .col-status {
    width: 36px;
    text-align: center;
  }

  .col-name {
    min-width: 180px;
  }

  .col-vendor {
    min-width: 120px;
  }

  .col-format {
    width: 60px;
  }

  .col-version {
    width: 80px;
  }

  .col-arch {
    width: 50px;
  }

  .col-size {
    width: 80px;
    text-align: right;
  }

  .col-actions {
    width: 36px;
  }

  .toggle {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    border: 2px solid var(--text-secondary);
    background: transparent;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    transition: all 0.15s;
  }

  .toggle.enabled {
    border-color: #34d399;
    background: #34d399;
  }

  .toggle.disabled {
    border-color: #6b7280;
    background: transparent;
  }

  .toggle-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: currentColor;
  }

  .toggle.enabled .toggle-dot {
    background: #fff;
  }

  .toggle.disabled .toggle-dot {
    background: #6b7280;
  }

  .badge {
    display: inline-block;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 11px;
    font-weight: 600;
  }

  .badge-audiounit {
    background: #1d4ed8;
    color: #fff;
  }

  .badge-vst3 {
    background: #7c3aed;
    color: #fff;
  }

  .arch-badge {
    display: inline-block;
    padding: 2px 5px;
    border-radius: 4px;
    font-size: 10px;
    font-weight: 700;
    font-family: monospace;
  }

  .arch-badge.arm {
    background: #065f46;
    color: #6ee7b7;
  }

  .arch-badge.intel {
    background: #78350f;
    color: #fcd34d;
  }

  .arch-badge.universal {
    background: #1e3a5f;
    color: #93c5fd;
  }

  .arch-badge.unknown {
    background: #374151;
    color: #9ca3af;
  }

  .btn-delete {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 15px;
    color: var(--text-secondary);
    padding: 2px 4px;
    border-radius: 4px;
    opacity: 0;
    transition: all 0.15s;
  }

  .plugin-row:hover .btn-delete {
    opacity: 1;
  }

  .btn-delete:hover {
    color: #ef4444;
    background: rgba(239, 68, 68, 0.1);
  }

  .empty-state {
    text-align: center;
    padding: 40px 20px !important;
    color: var(--text-secondary);
    font-size: 14px;
  }
</style>
