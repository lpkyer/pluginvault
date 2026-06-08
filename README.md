# PluginVault

**Manage your macOS audio plugins. Open source. Privacy first.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.96+-orange.svg)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-2-purple.svg)](https://v2.tauri.app)
[![Svelte](https://img.shields.io/badge/Svelte-5-orange.svg)](https://svelte.dev)
[![macOS](https://img.shields.io/badge/macOS-14+-black.svg)](https://apple.com/macos)

---

## Français

**PluginVault** est un gestionnaire de plugins audio gratuit et open source pour macOS. Il scanne tous vos plugins AU et VST3, vous permet de les activer/désactiver, de les supprimer et de naviguer votre bibliothèque en un coup d'œil.

### Fonctionnalités

- **Scan universel** — Détecte automatiquement tous les plugins AU (`.component`) et VST3 (`.vst3`) dans les dossiers système et utilisateur
- **Architecture CPU** — Détecte Apple Silicon, Intel 64, Universal 2 avec des badges couleur
- **Activer / Désactiver** — Basculez un plugin sans le supprimer (renommage avec suffixe `.disabled`)
- **Supprimer** — Effacez complètement un plugin du disque avec privilèges administrateur
- **Recherche & filtres** — Cherchez par nom, manufacturier ou bundle ID ; filtrez par format et manufacturier
- **Tri** — Cliquez sur les en-têtes de colonne pour trier par nom, version, taille, etc.
- **Détails** — Panneau latéral avec métadonnées complètes (version, architecture, chemin, bundle ID)
- **Persistant** — Le cache SQLite garde vos plugins entre les sessions
- **100 % local** — Aucune télémétrie, aucun appel réseau, vos données restent sur votre machine

### Captures d'écran

*(à venir)*

### Prérequis

- macOS 14 Sonoma ou plus récent
- Apple Silicon ou Intel

### Installation

```bash
# Depuis les releases (à venir)
# Téléchargez le .dmg depuis la page Releases

# Ou build depuis la source
git clone https://github.com/lpkyer/pluginvault.git
cd pluginvault
npm install
npm run tauri build
```

### Utilisation

1. Lancez **PluginVault**
2. Cliquez sur **Scan Plugins** pour analyser vos dossiers de plugins
3. Utilisez les toggles pour activer/désactiver un plugin
4. Cliquez sur une rangée pour voir les détails dans le panneau latéral
5. Utilisez **Delete from Disk** pour supprimer un plugin (mot de passe administrateur requis)

### Formats supportés

| Format | Extension | Dossier |
|--------|-----------|---------|
| Audio Unit (AU) | `.component` | `/Library/Audio/Plug-Ins/Components/` |
| VST3 | `.vst3` | `/Library/Audio/Plug-Ins/VST3/` |

### Build depuis la source

```bash
# Cloner
git clone https://github.com/lpkyer/pluginvault.git
cd pluginvault

# Installer les dépendances frontend
npm install

# Lancer en mode développement
npm run tauri dev

# Build de production
npm run tauri build

# Lancer les tests Rust
cd src-tauri && cargo test
```

### Stack technologique

| Couche | Technologie |
|--------|------------|
| Application | Tauri v2 |
| Frontend | Svelte 5 + TypeScript + Vite |
| Backend | Rust |
| Base de données | SQLite (rusqlite) |
| Détection architecture | Mach-O parsing natif |

---

## English

**PluginVault** is a free, open-source audio plugin manager for macOS. It scans all your AU and VST3 plugins, lets you enable/disable them, delete them from disk, and browse your library at a glance.

### Features

- **Universal scanner** — Automatically detects all AU (`.component`) and VST3 (`.vst3`) plugins from system and user directories
- **CPU architecture** — Detects Apple Silicon, Intel 64, Universal 2 with colour-coded badges
- **Enable / Disable** — Toggle a plugin without deleting it (renamed with `.disabled` suffix)
- **Delete** — Permanently remove a plugin from disk using administrator privileges
- **Search & filters** — Search by name, vendor, or bundle ID; filter by format and vendor
- **Sorting** — Click column headers to sort by name, version, size, and more
- **Details** — Sidebar with full metadata (version, architecture, path, bundle ID)
- **Persistent** — SQLite cache keeps your plugins across sessions
- **100 % local** — No telemetry, no network calls, your data stays on your machine

### Screenshots

*(coming soon)*

### Requirements

- macOS 14 Sonoma or later
- Apple Silicon or Intel

### Installation

```bash
# From releases (coming soon)
# Download the .dmg from the Releases page

# Or build from source
git clone https://github.com/lpkyer/pluginvault.git
cd pluginvault
npm install
npm run tauri build
```

### Usage

1. Launch **PluginVault**
2. Click **Scan Plugins** to scan your plugin directories
3. Use the toggles to enable/disable a plugin
4. Click a row to view details in the sidebar panel
5. Use **Delete from Disk** to permanently remove a plugin (admin password required)

### Supported Formats

| Format | Extension | Location |
|--------|-----------|----------|
| Audio Unit (AU) | `.component` | `/Library/Audio/Plug-Ins/Components/` |
| VST3 | `.vst3` | `/Library/Audio/Plug-Ins/VST3/` |

### Building from Source

```bash
# Clone
git clone https://github.com/lpkyer/pluginvault.git
cd pluginvault

# Install frontend dependencies
npm install

# Run in development mode
npm run tauri dev

# Production build
npm run tauri build

# Run Rust tests
cd src-tauri && cargo test
```

### Tech Stack

| Layer | Technology |
|-------|------------|
| Application shell | Tauri v2 |
| Frontend | Svelte 5 + TypeScript + Vite |
| Backend | Rust |
| Database | SQLite (rusqlite) |
| Arch detection | Native Mach-O parsing |

---

## License

MIT — see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Feel free to open issues and pull requests on [GitHub](https://github.com/lpkyer/pluginvault).
