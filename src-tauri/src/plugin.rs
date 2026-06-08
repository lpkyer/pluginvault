use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginFormat {
    AudioUnit,
    Vst2,
    Vst3,
    Aax,
    Clap,
}

impl PluginFormat {
    pub fn label(&self) -> &str {
        match self {
            PluginFormat::AudioUnit => "AU",
            PluginFormat::Vst2 => "VST2",
            PluginFormat::Vst3 => "VST3",
            PluginFormat::Aax => "AAX",
            PluginFormat::Clap => "CLAP",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PluginArch {
    AppleSilicon,
    Intel,
    Universal,
    Unknown,
}

impl PluginArch {
    pub fn label(&self) -> &str {
        match self {
            PluginArch::AppleSilicon => "Apple Silicon",
            PluginArch::Intel => "Intel 64",
            PluginArch::Universal => "Universal",
            PluginArch::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub vendor: String,
    pub version: String,
    pub format: PluginFormat,
    pub path: String,
    pub bundle_id: String,
    pub arch: PluginArch,
    pub size_bytes: u64,
    pub enabled: bool,
}

impl Plugin {
    #[allow(dead_code)]
    pub fn disabled_path(&self) -> String {
        format!("{}.disabled", self.path)
    }

    #[allow(dead_code)]
    pub fn original_path(&self) -> String {
        if self.path.ends_with(".disabled") {
            self.path.trim_end_matches(".disabled").to_string()
        } else {
            self.path.clone()
        }
    }
}
