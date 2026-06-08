export interface Plugin {
  id: string;
  name: string;
  vendor: string;
  version: string;
  format: "AudioUnit" | "Vst2" | "Vst3" | "Aax" | "Clap";
  path: string;
  bundle_id: string;
  arch: "AppleSilicon" | "Intel" | "Universal" | "Unknown";
  size_bytes: number;
  enabled: boolean;
}

export type SortField = "name" | "vendor" | "version" | "format" | "arch" | "size_bytes";
export type SortDir = "asc" | "desc";
