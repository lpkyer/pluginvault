use crate::plugin::PluginArch;
use std::fs;
use std::fs::File;

const FAT_MAGIC: u32 = 0xcafebabe;
const FAT_CIGAM: u32 = 0xbebafeca;
const MAGIC_64: u32 = 0xfeedfacf;
const MAGIC_32: u32 = 0xfeedface;

#[allow(dead_code)]
#[repr(C)]
struct FatHeader {
    magic: u32,
    nfat_arch: u32,
}

#[allow(dead_code)]
#[repr(C)]
struct FatArch {
    cputype: u32,
    cpusubtype: u32,
    offset: u32,
    size: u32,
    align: u32,
}

const CPU_TYPE_ARM64: u32 = 0x0100000c;
const CPU_TYPE_X86_64: u32 = 0x01000007;

pub fn detect_architecture(binary_path: &str) -> PluginArch {
    let data = match fs::read(binary_path) {
        Ok(d) => d,
        Err(_) => return PluginArch::Unknown,
    };

    if data.len() < 8 {
        return PluginArch::Unknown;
    }

    let magic = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);

    match magic {
        FAT_MAGIC | FAT_CIGAM => {
            let nfat = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
            let mut has_arm64 = false;
            let mut has_x86_64 = false;

            for i in 0..nfat as usize {
                let offset = 8 + i * 20;
                if offset + 20 > data.len() {
                    break;
                }
                let cputype = u32::from_be_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]);
                match cputype {
                    CPU_TYPE_ARM64 => has_arm64 = true,
                    CPU_TYPE_X86_64 => has_x86_64 = true,
                    _ => {}
                }
            }

            if has_arm64 && has_x86_64 {
                PluginArch::Universal
            } else if has_arm64 {
                PluginArch::AppleSilicon
            } else if has_x86_64 {
                PluginArch::Intel
            } else {
                PluginArch::Unknown
            }
        }
        MAGIC_64 | MAGIC_32 => {
            if magic == MAGIC_64 || magic == MAGIC_32 {
                let cputype = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
                match cputype {
                    CPU_TYPE_ARM64 => PluginArch::AppleSilicon,
                    CPU_TYPE_X86_64 => PluginArch::Intel,
                    _ => PluginArch::Unknown,
                }
            } else {
                PluginArch::Unknown
            }
        }
        _ => PluginArch::Unknown,
    }
}

pub fn find_main_binary(bundle_path: &str) -> Option<String> {
    let plist_path = format!("{}/Contents/Info.plist", bundle_path);
    let macos_dir = format!("{}/Contents/MacOS", bundle_path);

    if !std::path::Path::new(&plist_path).exists() {
        let macos_dir2 = format!("{}/Contents/MacOS", bundle_path);
        if std::path::Path::new(&macos_dir2).exists() {
            if let Ok(entries) = fs::read_dir(&macos_dir2) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() || path.is_symlink() {
                        return Some(path.to_string_lossy().to_string());
                    }
                }
            }
        }
        return None;
    }

    if let Ok(file) = File::open(&plist_path) {
        if let Ok(value) = plist::from_reader::<_, plist::Dictionary>(file) {
            if let Some(exec) = value.get("CFBundleExecutable").and_then(|v| v.as_string()) {
                let binary = format!("{}/Contents/MacOS/{}", bundle_path, exec);
                if std::path::Path::new(&binary).exists() {
                    return Some(binary);
                }
            }
        }
    }

    if std::path::Path::new(&macos_dir).exists() {
        if let Ok(entries) = fs::read_dir(&macos_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() || path.is_symlink() {
                    return Some(path.to_string_lossy().to_string());
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_fat_binary() {
        let arch = detect_architecture("/usr/bin/env");
        assert!(
            matches!(arch, PluginArch::Universal | PluginArch::AppleSilicon | PluginArch::Intel),
            "system binaries should be detected, got {:?}", arch
        );
    }

    #[test]
    fn test_detect_nonexistent_binary() {
        let arch = detect_architecture("/nonexistent/path");
        assert_eq!(arch, PluginArch::Unknown);
    }

    #[test]
    fn test_detect_empty_file() {
        let dir = std::env::temp_dir().join("pluginvault_arch_test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("empty.bin");
        std::fs::write(&path, b"").unwrap();
        let arch = detect_architecture(path.to_str().unwrap());
        assert_eq!(arch, PluginArch::Unknown);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_find_main_binary_no_plist() {
        let result = find_main_binary("/nonexistent/bundle.vst3");
        assert!(result.is_none());
    }

    #[test]
    fn test_find_main_binary_with_macos_dir() {
        let dir = std::env::temp_dir().join("pluginvault_bundle_test.vst3");
        let macos_dir = dir.join("Contents/MacOS");
        std::fs::create_dir_all(&macos_dir).unwrap();
        std::fs::write(macos_dir.join("MyPlugin"), "binary data").unwrap();
        let result = find_main_binary(dir.to_str().unwrap());
        assert!(result.is_some());
        assert!(result.unwrap().ends_with("MyPlugin"));
        std::fs::remove_dir_all(&dir).ok();
    }
}
