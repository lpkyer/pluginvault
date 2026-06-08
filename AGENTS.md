# PluginVault

Audio plugin manager for macOS. Scans, organizes, enables/disables, and deletes VST3 and AU plugins.

## Stack

| Layer | Technology |
|-------|-----------|
| Desktop shell | Tauri v2 (Rust backend + webview frontend) |
| Frontend | Svelte 5 + TypeScript + Vite |
| Backend | Rust (scan, arch detection, SQLite, file ops) |
| Database | SQLite via `rusqlite` (bundled) |
| UI Framework | Svelte 5 (runes/mount API) |

## Project Structure

```
pluginvault/
├── src/                          # Frontend (Svelte)
│   ├── main.ts                   # Entry point
│   ├── App.svelte                # Root layout (header, error banner, main content)
│   └── lib/
│       ├── types.ts              # Plugin, SortField, SortDir interfaces
│       ├── commands.ts           # Tauri invoke wrappers (scan, get, toggle, delete, reveal)
│       ├── stores.ts             # Svelte stores (plugins, filters, sort, loading, errors)
│       └── components/
│           ├── Toolbar.svelte     # Scan/Refresh buttons, search, format/vendor filters
│           ├── PluginTable.svelte # Sortable table with enable toggle, delete button
│           └── PluginDetail.svelte# Sidebar with metadata, enable/disable, reveal, delete
├── src-tauri/                    # Backend (Rust)
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/default.json # Tauri v2 permissions (shell)
│   ├── icons/                    # App icons (PNG, ICNS, ICO)
│   └── src/
│       ├── main.rs               # Binary entry point
│       ├── lib.rs                # Tauri app setup, command handlers, state management
│       ├── plugin.rs             # Data models: Plugin, PluginFormat, PluginArch
│       ├── scanner.rs            # Filesystem scanner for AU/VST3 bundles
│       ├── arch.rs               # Mach-O fat/thin binary architecture detection
│       ├── db.rs                 # SQLite persistence (upsert, query, update, delete)
│       └── operations.rs         # Enable/disable (rename) and delete (osascript admin)
├── index.html
├── package.json
├── vite.config.ts
├── svelte.config.js
├── tsconfig.json
└── AGENTS.md                     # This file
```

## Commands

```bash
# Development (hot-reload both Rust and frontend)
npm run tauri dev

# Production build
npm run tauri build

# Run Rust tests only
cd src-tauri && cargo test

# Build frontend standalone
npm run build

# Build Rust backend standalone
cd src-tauri && cargo build
```

## Tauri Commands (invoke bridge)

| Command | Parameters | Returns | Description |
|---------|-----------|---------|-------------|
| `scan_plugins` | none | `Plugin[]` | Scan all standard AU/VST3 directories, parse plists, detect arch, persist to SQLite |
| `get_plugins` | none | `Plugin[]` | Load all cached plugins from SQLite |
| `toggle_plugin` | `id: string, enable: bool` | `bool` | Rename bundle to `.disabled` suffix or back |
| `delete_plugin` | `id: string` | `void` | Delete plugin from disk via osascript with admin privileges |
| `reveal_in_finder` | `path: string` | `void` | Open `-R` in Finder via `open` command |

## Plugin Scanning

### Scan Paths
- `/Library/Audio/Plug-Ins/Components/` — AU (`.component`)
- `/Library/Audio/Plug-Ins/VST3/` — VST3 (`.vst3`)
- `~/Library/Audio/Plug-Ins/Components/` — User-level AU
- `~/Library/Audio/Plug-Ins/VST3/` — User-level VST3

### Scan Flow
1. Walk `WalkDir` over each path
2. For each `.component`/`.vst3` directory (including `.disabled` variants):
   - Parse `Contents/Info.plist` → `CFBundleName`, `CFBundleIdentifier`, `CFBundleShortVersionString`, `NSHumanReadableCopyright`
   - Detect architecture from Mach-O header of main binary → AppleSilicon / Intel / Universal / Unknown
   - Calculate directory size via recursive walk
   - Determine enabled status from filename suffix
3. Upsert all results into SQLite

### Architecture Detection
Reads Mach-O header bytes directly (no external deps):
- `0xcafebabe` / `0xbebafeca` → fat binary → iterate archs → Universal / AppleSilicon / Intel
- `0xfeedfacf` (64-bit) / `0xfeedface` (32-bit) → thin binary → read cputype
- CPU type `0x0100000c` = ARM64, `0x01000007` = x86_64

## Enable/Disable

Renaming the bundle directory:
- Disable: `Plugin.component` → `Plugin.component.disabled`
- Enable: `Plugin.component.disabled` → `Plugin.component`

DAWs skip directories not matching their expected extension.

## Delete

Uses AppleScript with admin privileges via `osascript`:
```
do shell script "rm -rf '/path/to/plugin.vst3'" with administrator privileges
```
This triggers the standard macOS authentication dialog.

## Error Handling

Errors from Tauri commands are displayed in a red dismissable banner at the top of the app. The `errorMessage` Svelte store is set in catch blocks and cleared on click.

## Styling

Dark theme via CSS custom properties in `App.svelte`:
- `--bg: #0f1117`
- `--surface: #1a1b23`
- `--border: #2a2b35`
- `--text: #e1e2e8`
- `--accent: #6366f1`

## Data Flow

```
User clicks Scan
  → Toolbar.handleScan()
  → scanPlugins() [invoke Rust]
  → scanner::scan_plugins() [filesystem]
  → db.upsert_plugins() [SQLite]
  → Returns Vec<Plugin> to frontend
  → getPlugins() [reload from DB]
  → $plugins store = result
  → filteredPlugins derived store re-computes (search, format, vendor, sort)
  → PluginTable re-renders
```

```
User clicks enable/disable toggle
  → togglePlugin(id, !enabled) [invoke Rust]
  → operations::toggle_plugin() [rename on disk]
  → db.update_enabled() [SQLite]
  → $plugins store updated optimistically
```

```
User clicks delete
  → confirm dialog
  → deletePlugin(id) [invoke Rust]
  → operations::delete_plugin() [osascript admin]
  → db.delete_plugin() [SQLite]
  → $plugins store filtered
```

## Tests

10 Rust unit tests in `arch.rs` and `operations.rs`:

```bash
cd src-tauri && cargo test
```

### operations tests
- `test_toggle_enable_disable_roundtrip` — create dir, disable, re-enable, verify filesystem
- `test_toggle_nonexistent` — toggle nonexistent path → error
- `test_delete_existing` — create dir, delete, verify gone
- `test_delete_nonexistent` — delete nonexistent path → "Plugin not found on disk"
- `test_delete_disabled_form` — delete plugin that's in `.disabled` state

### arch tests
- `test_detect_fat_binary` — `/usr/bin/env` → Universal
- `test_detect_nonexistent_binary` — missing file → Unknown
- `test_detect_empty_file` — empty file → Unknown
- `test_find_main_binary_with_macos_dir` — bundle with MacOS dir with binary → found
- `test_find_main_binary_no_plist` — bundle without Info.plist → None

## Key Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Plugin identity key | Bundle ID (`CFBundleIdentifier`) | Survives moves/renames, unique across formats |
| Architecture detection | Raw Mach-O header parsing | No external deps, instant, accurate |
| Plugin cache | SQLite via rusqlite | Fast relaunch, standard querying |
| Enable/disable method | Rename bundle (`.disabled` suffix) | Non-destructive, reversible, no file content changes |
| Delete method | osascript with admin privileges | macOS-native auth dialog, handles root-owned files |
| Scan execution | `tauri::async_runtime::spawn_blocking` | Prevents UI freeze during filesystem scan |
| State management | Svelte stores `writable` + `derived` | Simple reactive updates from Tauri events |
| Frontend framework | Svelte 5 | Lightweight, excellent Tauri DX, minimal boilerplate |

## Roadmap (future)

- [ ] Support for CLAP, AAX, VST2 formats
- [ ] DAW project scanning (Ableton, Logic, Pro Tools)
- [ ] Update checking (KVR Audio API)
- [ ] License vault (encrypted local storage)
- [ ] Disk usage analytics / charts
- [ ] Plugin offloading (move to vault folder)
- [ ] Real-time filesystem monitoring (FSEvents)
- [ ] Export library as CSV / PDF
- [ ] System profile sharing / comparison
- [ ] Dark/light theme toggle
