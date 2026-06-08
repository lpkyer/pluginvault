import { invoke } from "@tauri-apps/api/core";
import type { Plugin } from "./types";

export async function scanPlugins(): Promise<Plugin[]> {
  return invoke<Plugin[]>("scan_plugins");
}

export async function getPlugins(): Promise<Plugin[]> {
  return invoke<Plugin[]>("get_plugins");
}

export async function togglePlugin(id: string, enable: boolean): Promise<boolean> {
  return invoke<boolean>("toggle_plugin", { id, enable });
}

export async function deletePlugin(id: string): Promise<void> {
  return invoke<void>("delete_plugin", { id });
}

export async function revealInFinder(path: string): Promise<void> {
  return invoke<void>("reveal_in_finder", { path });
}
