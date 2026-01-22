use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::DriverResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DpiStage {
    pub dpi_x: u16,
    pub dpi_y: u16,
    pub stage: u8,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MouseSettings {
    #[serde(default = "default_dpi_x")]
    pub dpi_x: u16,
    #[serde(default = "default_dpi_y")]
    pub dpi_y: u16,
    #[serde(default = "default_polling_rate")]
    pub polling_rate: u16,
    #[serde(default = "default_rgb_color")]
    pub rgb_color: [u8; 3],
    #[serde(default = "default_brightness")]
    pub brightness: u8,
    #[serde(default = "default_scroll_inverted")]
    pub scroll_inverted: bool,
    #[serde(default = "default_smart_wheel_enabled")]
    pub smart_wheel_enabled: bool,
    #[serde(default = "default_auto_update")]
    pub auto_update: bool,
    #[serde(default = "default_dpi_stages")]
    pub dpi_stages: Vec<DpiStage>,
}

fn default_dpi_x() -> u16 { 3200 }
fn default_dpi_y() -> u16 { 3200 }
fn default_polling_rate() -> u16 { 1000 }
fn default_rgb_color() -> [u8; 3] { [255, 255, 255] }
fn default_brightness() -> u8 { 100 }
fn default_scroll_inverted() -> bool { false }
fn default_smart_wheel_enabled() -> bool { false }
fn default_auto_update() -> bool { false }
fn default_dpi_stages() -> Vec<DpiStage> {
    vec![
        DpiStage { dpi_x: 400, dpi_y: 400, stage: 1, active: false },
        DpiStage { dpi_x: 800, dpi_y: 800, stage: 2, active: false },
        DpiStage { dpi_x: 1600, dpi_y: 1600, stage: 3, active: false },
        DpiStage { dpi_x: 3200, dpi_y: 3200, stage: 4, active: true },
        DpiStage { dpi_x: 6400, dpi_y: 6400, stage: 5, active: false },
    ]
}

impl Default for MouseSettings {
    fn default() -> Self {
        Self {
            dpi_x: 3200,
            dpi_y: 3200,
            polling_rate: 1000,
            rgb_color: [255, 255, 255],
            brightness: 100,
            scroll_inverted: false,
            smart_wheel_enabled: false,
            auto_update: false,
            dpi_stages: vec![
                DpiStage { dpi_x: 400, dpi_y: 400, stage: 1, active: false },
                DpiStage { dpi_x: 800, dpi_y: 800, stage: 2, active: false },
                DpiStage { dpi_x: 1600, dpi_y: 1600, stage: 3, active: false },
                DpiStage { dpi_x: 3200, dpi_y: 3200, stage: 4, active: true },
                DpiStage { dpi_x: 6400, dpi_y: 6400, stage: 5, active: false },
            ],
        }
    }
}

impl MouseSettings {
    /// Load settings from a JSON file. Returns default settings if file doesn't exist.
    pub fn load(path: &Path) -> DriverResult<Self> {
        if !path.exists() {
            log::info!("Settings file not found at {:?}, using defaults", path);
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(path)?;

        let settings: MouseSettings = serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse settings file: {}", e))?;

        log::info!("Loaded settings from {:?}", path);
        Ok(settings)
    }

    /// Save settings to a JSON file.
    pub fn save(&self, path: &Path) -> DriverResult<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;

        fs::write(path, json)?;

        log::info!("Saved settings to {:?}", path);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_settings() {
        let settings = MouseSettings::default();
        assert_eq!(settings.dpi_x, 3200);
        assert_eq!(settings.dpi_y, 3200);
        assert_eq!(settings.polling_rate, 1000);
        assert_eq!(settings.rgb_color, [255, 255, 255]);
        assert_eq!(settings.brightness, 100);
        assert_eq!(settings.scroll_inverted, false);
        assert_eq!(settings.smart_wheel_enabled, false);
        assert_eq!(settings.auto_update, false);
        assert_eq!(settings.dpi_stages.len(), 5);
    }

    #[test]
    fn test_save_and_load_settings() {
        let temp_dir = env::temp_dir();
        let settings_path = temp_dir.join("test_razer_settings.json");

        // Clean up from previous test runs
        let _ = fs::remove_file(&settings_path);

        // Create custom settings
        let original_settings = MouseSettings {
            dpi_x: 1600,
            dpi_y: 1600,
            polling_rate: 500,
            rgb_color: [255, 0, 0],
            brightness: 50,
            scroll_inverted: true,
            smart_wheel_enabled: false,
            auto_update: true,
            dpi_stages: vec![],
        };

        // Save settings
        original_settings.save(&settings_path).unwrap();

        // Load settings
        let loaded_settings = MouseSettings::load(&settings_path).unwrap();

        // Verify
        assert_eq!(loaded_settings.dpi_x, 1600);
        assert_eq!(loaded_settings.dpi_y, 1600);
        assert_eq!(loaded_settings.polling_rate, 500);
        assert_eq!(loaded_settings.rgb_color, [255, 0, 0]);
        assert_eq!(loaded_settings.brightness, 50);
        assert_eq!(loaded_settings.scroll_inverted, true);
        assert_eq!(loaded_settings.auto_update, true);

        // Clean up
        let _ = fs::remove_file(&settings_path);
    }

    #[test]
    fn test_load_nonexistent_file_returns_defaults() {
        let temp_dir = env::temp_dir();
        let settings_path = temp_dir.join("nonexistent_settings.json");

        // Ensure file doesn't exist
        let _ = fs::remove_file(&settings_path);

        // Load should return defaults
        let settings = MouseSettings::load(&settings_path).unwrap();
        assert_eq!(settings.dpi_x, 3200);
        assert_eq!(settings.polling_rate, 1000);
    }
}
