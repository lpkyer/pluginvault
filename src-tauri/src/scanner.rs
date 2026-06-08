use crate::arch;
use crate::plugin::{Plugin, PluginArch, PluginFormat};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

const SCAN_PATHS: &[&str] = &[
    "/Library/Audio/Plug-Ins/Components",
    "/Library/Audio/Plug-Ins/VST",
    "/Library/Audio/Plug-Ins/VST3",
    "/Library/Audio/Plug-Ins/CLAP",
    "/Library/Application Support/Avid/Audio/Plug-Ins",
];

fn user_library() -> Option<String> {
    if let Ok(home) = std::env::var("HOME") {
        Some(format!("{}/Library/Audio/Plug-Ins", home))
    } else {
        None
    }
}

pub fn get_scan_directories() -> Vec<String> {
    let mut dirs: Vec<String> = SCAN_PATHS.iter().map(|s| s.to_string()).collect();
    if let Some(user_lib) = user_library() {
        dirs.push(format!("{}/Components", user_lib));
        dirs.push(format!("{}/VST", user_lib));
        dirs.push(format!("{}/VST3", user_lib));
        dirs.push(format!("{}/CLAP", user_lib));
    }
    dirs
}

fn get_directory_size(path: &str) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.metadata().ok())
        .filter(|m| m.is_file())
        .map(|m| m.len())
        .sum()
}

fn read_plist_string(dict: &plist::Dictionary, key: &str) -> String {
    dict.get(key)
        .and_then(|v| v.as_string())
        .map(|s| s.to_string())
        .unwrap_or_default()
}

fn scan_bundle(path: &str, format: PluginFormat) -> Option<Plugin> {
    let plist_path = format!("{}/Contents/Info.plist", path);
    if !Path::new(&plist_path).exists() {
        return None;
    }

    let file = match fs::File::open(&plist_path) {
        Ok(f) => f,
        Err(_) => return None,
    };

    let dict: plist::Dictionary = match plist::from_reader(file) {
        Ok(d) => d,
        Err(_) => return None,
    };

    let name = read_plist_string(&dict, "CFBundleName");
    let bundle_id = read_plist_string(&dict, "CFBundleIdentifier");
    let version = read_plist_string(&dict, "CFBundleShortVersionString");
    let copyright = read_plist_string(&dict, "NSHumanReadableCopyright");

    if name.is_empty() && bundle_id.is_empty() {
        return None;
    }

    let vendor = if copyright.contains('©') {
        copyright
            .split('©')
            .nth(1)
            .unwrap_or(&copyright)
            .trim()
            .trim_end_matches('.')
            .trim()
            .to_string()
    } else {
        bundle_id
            .split('.')
            .nth(1)
            .unwrap_or("Unknown")
            .to_string()
    };

    let name = if name.is_empty() {
        bundle_id
            .split('.')
            .last()
            .unwrap_or(&bundle_id)
            .to_string()
    } else {
        name
    };

    let arch = match arch::find_main_binary(path) {
        Some(bin_path) => arch::detect_architecture(&bin_path),
        None => PluginArch::Unknown,
    };

    let size = get_directory_size(path);
    let enabled = !path.ends_with(".disabled");
    let id = bundle_id.clone();

    Some(Plugin {
        id,
        name,
        vendor,
        version,
        format,
        path: path.to_string(),
        bundle_id,
        arch,
        size_bytes: size,
        enabled,
    })
}

pub fn scan_plugins() -> Vec<Plugin> {
    let directories = get_scan_directories();
    let mut plugins = Vec::new();

    for dir in &directories {
        let dir_path = Path::new(dir);
        if !dir_path.exists() {
            continue;
        }

        let (format, extensions) = if dir.contains("Components") {
            (PluginFormat::AudioUnit, &["component"][..])
        } else if dir.contains("/VST3") || dir.ends_with("VST3") {
            (PluginFormat::Vst3, &["vst3"][..])
        } else if dir.contains("/VST") || dir.ends_with("VST") {
            (PluginFormat::Vst2, &["vst"][..])
        } else if dir.contains("/CLAP") || dir.ends_with("CLAP") {
            (PluginFormat::Clap, &["clap"][..])
        } else if dir.contains("Avid") || dir.contains("AAX") {
            (PluginFormat::Aax, &["aaxplugin"][..])
        } else {
            continue;
        };

        let entries = match fs::read_dir(dir_path) {
            Ok(e) => e,
            Err(_) => continue,
        };

        for entry in entries.flatten() {
            let entry_path = entry.path();
            let path_str = entry_path.to_string_lossy().to_string();

            if entry_path.is_symlink() {
                continue;
            }

            if !entry_path.is_dir() {
                continue;
            }

            let matches = extensions.iter().any(|ext| {
                path_str.ends_with(&format!(".{}", ext))
                    || path_str.ends_with(&format!(".{}.disabled", ext))
            });

            if !matches {
                continue;
            }

            if let Some(plugin) = scan_bundle(&path_str, format.clone()) {
                plugins.push(plugin);
            }
        }
    }

    plugins
}
