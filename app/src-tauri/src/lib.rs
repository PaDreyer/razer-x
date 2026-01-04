mod handler;
mod types;
mod mouse;

use tauri::Manager;
use tauri::path::BaseDirectory;
use handler::{
    get_device_information,
    set_device_backlight_brightness,
    set_device_matrix_backlight_static,
    set_device_polling_rate,
    get_device_led_rgb,
};
use types::{DeviceCollection, DeviceInfo};
use crate::handler::{get_device_backlight_brightness, set_device_dpi};

pub struct Application {
    pub app: tauri::App,
}


impl Application {
    pub fn new(app: tauri::App) -> Self {
        Application {
            app,
        }
    }

    pub fn run(self)  {
        self.app.run(|_, _| {})
    }
}

pub fn create_app() -> Application {
    Application::new(
        tauri::Builder::default()
            .plugin(tauri_plugin_opener::init())
            .invoke_handler(tauri::generate_handler![
                get_device_information,
                set_device_matrix_backlight_static,
                set_device_backlight_brightness,
                get_device_backlight_brightness,
                set_device_polling_rate,
                set_device_dpi,
                get_device_led_rgb])
            .build(tauri::generate_context!())
            .expect("Failed to build Tauri application"),
    )
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = create_app();

    let path = app.app.path().resolve("supported_devices", BaseDirectory::Resource).unwrap();
    let mut device_collection: Vec<DeviceInfo> = Vec::new();
    if path.exists() {
        if let Ok(entries) = std::fs::read_dir(&path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path_buf = entry.path().clone();
                    let file_path = file_path_buf.to_str().unwrap_or_default();
                    let file_content = std::fs::read_to_string(file_path).unwrap();
                    let device_info: DeviceInfo = serde_json::from_str(&file_content).unwrap();
                    device_collection.push(device_info);
                } else {
                    println!("Failed to read entry");
                }
            }
        } else {
            println!("Failed to read directory: {}", path.display());
        }
    }

    app.app
        .manage(DeviceCollection {
            devices: device_collection,
        });

    app.run();
}

