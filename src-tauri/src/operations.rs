use std::fs;
use std::path::Path;

pub fn toggle_plugin(path: &str, enable: bool) -> Result<bool, String> {
    let disabled_path = format!("{}.disabled", path);

    if enable {
        if Path::new(&disabled_path).exists() {
            fs::rename(&disabled_path, path).map_err(|e| format!("Failed to enable plugin: {}", e))?;
            Ok(true)
        } else {
            Err("Plugin is not disabled".to_string())
        }
    } else {
        if Path::new(path).exists() {
            fs::rename(path, &disabled_path).map_err(|e| format!("Failed to disable plugin: {}", e))?;
            Ok(false)
        } else if Path::new(&disabled_path).exists() {
            Err("Plugin is already disabled".to_string())
        } else {
            Err("Plugin not found".to_string())
        }
    }
}

pub fn delete_plugin(path: &str) -> Result<(), String> {
    let target = if Path::new(path).exists() {
        path.to_string()
    } else {
        let disabled = format!("{}.disabled", path);
        if Path::new(&disabled).exists() {
            disabled
        } else {
            return Err("Plugin not found on disk".to_string());
        }
    };

    let escaped = target.replace('\'', "'\\''");
    let script = format!(
        "do shell script \"rm -rf '{}'\" with administrator privileges",
        escaped
    );

    let output = std::process::Command::new("osascript")
        .args(["-e", &script])
        .output()
        .map_err(|e| format!("Failed to execute delete: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("User cancelled") || stderr.contains("Authorization cancelled") {
            return Err("Delete cancelled".to_string());
        }
        return Err(format!("Delete failed: {}", stderr));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_enable_disable_roundtrip() {
        let dir = std::env::temp_dir().join("pluginvault_test_toggle");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.to_str().unwrap().to_string();

        // disable
        let result = toggle_plugin(&path, false);
        assert!(result.is_ok(), "disable should succeed: {:?}", result);
        assert!(!result.unwrap());
        assert!(Path::new(&format!("{}.disabled", path)).exists());
        assert!(!Path::new(&path).exists());

        // re-enable
        let result = toggle_plugin(&path, true);
        assert!(result.is_ok(), "re-enable should succeed: {:?}", result);
        assert!(result.unwrap());
        assert!(Path::new(&path).exists());

        // cleanup
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_toggle_nonexistent() {
        let path = "/tmp/pluginvault_test_nonexistent/foo.component";
        let result = toggle_plugin(path, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_existing() {
        let dir = std::env::temp_dir().join("pluginvault_test_delete");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.to_str().unwrap().to_string();

        let result = delete_plugin(&path);
        assert!(result.is_ok(), "delete should succeed: {:?}", result);
        assert!(!Path::new(&path).exists());
    }

    #[test]
    fn test_delete_nonexistent() {
        let path = "/tmp/pluginvault_test_nonexistent_12345/foo.vst3";
        let result = delete_plugin(path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Plugin not found on disk");
    }

    #[test]
    fn test_delete_disabled_form() {
        let dir = std::env::temp_dir().join("pluginvault_test_disabled");
        let disabled_dir = format!("{}.disabled", dir.to_str().unwrap());
        fs::create_dir_all(&disabled_dir).unwrap();
        let path = dir.to_str().unwrap().to_string();

        let result = delete_plugin(&path);
        assert!(result.is_ok(), "delete disabled plugin: {:?}", result);
        assert!(!Path::new(&disabled_dir).exists());
    }
}
