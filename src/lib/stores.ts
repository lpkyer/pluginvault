import { writable, derived } from "svelte/store";
import type { Plugin, SortField, SortDir } from "./types";

export const plugins = writable<Plugin[]>([]);
export const selectedPluginId = writable<string | null>(null);
export const searchQuery = writable("");
export const sortField = writable<SortField>("name");
export const sortDir = writable<SortDir>("asc");
export const isScanning = writable(false);
export const errorMessage = writable<string | null>(null);
export const formatFilter = writable<string>("all");
export const vendorFilter = writable<string>("all");

export const vendors = derived(plugins, ($plugins) => {
  const set = new Set<string>();
  for (const p of $plugins) {
    if (p.vendor) set.add(p.vendor);
  }
  return Array.from(set).sort();
});

export const selectedPlugin = derived(
  [plugins, selectedPluginId],
  ([$plugins, $selectedPluginId]) => {
    if (!$selectedPluginId) return null;
    return $plugins.find((p) => p.id === $selectedPluginId) ?? null;
  }
);

export const filteredPlugins = derived(
  [plugins, searchQuery, formatFilter, vendorFilter, sortField, sortDir],
  ([$plugins, $searchQuery, $formatFilter, $vendorFilter, $sortField, $sortDir]) => {
    let result = $plugins;

    if ($formatFilter !== "all") {
      result = result.filter((p) => p.format === $formatFilter);
    }

    if ($vendorFilter !== "all") {
      result = result.filter((p) => p.vendor === $vendorFilter);
    }

    if ($searchQuery) {
      const q = $searchQuery.toLowerCase();
      result = result.filter(
        (p) =>
          p.name.toLowerCase().includes(q) ||
          p.vendor.toLowerCase().includes(q) ||
          p.bundle_id.toLowerCase().includes(q)
      );
    }

    result = result.sort((a, b) => {
      let cmp = 0;
      switch ($sortField) {
        case "name":
          cmp = a.name.localeCompare(b.name);
          break;
        case "vendor":
          cmp = a.vendor.localeCompare(b.vendor);
          break;
        case "version":
          cmp = a.version.localeCompare(b.version);
          break;
        case "format":
          cmp = a.format.localeCompare(b.format);
          break;
        case "arch":
          cmp = a.arch.localeCompare(b.arch);
          break;
        case "size_bytes":
          cmp = a.size_bytes - b.size_bytes;
          break;
      }
      return $sortDir === "asc" ? cmp : -cmp;
    });

    return result;
  }
);
