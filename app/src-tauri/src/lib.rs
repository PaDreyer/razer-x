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
    get_target_os,
    set_mouse_wheel_inverted,
    get_device_backlight_brightness,
    set_device_dpi,
    get_device_dpi_stages,
    set_device_dpi_stages,
    get_device_battery_status,
};
use types::{DeviceCollection, DeviceInfo};

pub struct Application {
    pub app: tauri::App,
}


impl Application {
    pub fn new(app: tauri::App) -> Self {
        Application {
            app,
        }
    }

    pub fn run(self) -> !  {
        self.app.run(|app, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            tauri::RunEvent::Reopen { has_visible_windows , .. } => {
                let window = app.get_webview_window("main").unwrap();
                if !has_visible_windows {
                    if let Err(e) = window.show() {
                        eprintln!("Failed to show window: {}", e);
                    }
                }
            }
            _ => {}
        });
        loop {} // This will never be reached, but is needed to satisfy the return type
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
                get_device_led_rgb,
                get_target_os,
                set_mouse_wheel_inverted,
                get_device_dpi_stages,
                set_device_dpi_stages,
                get_device_battery_status,])
            .on_window_event(|window, event| match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    #[cfg(not(target_os = "macos"))] {
                        event.window().hide().unwrap();
                    }

                    #[cfg(target_os = "macos")] {
                        window.hide().unwrap();
                        //tauri::AppHandle::manager().get_window("main").unwrap().minimize().unwrap();
                    }
                    api.prevent_close();
                }
                _ => {}
            })
            .setup(|app| {
                let open_ui = tauri::menu::MenuItem::with_id(app, "open_ui", "Open UI", true, None::<&str>)?;
                let sync_settings = tauri::menu::MenuItem::with_id(app, "sync", "Sync settings", true, None::<&str>)?;
                let separator = tauri::menu::PredefinedMenuItem::separator(app)?;
                let about = tauri::menu::PredefinedMenuItem::about(
                    app,
                    Some("About Razer-X"),
                    Some(tauri::menu::AboutMetadata {
                        name: Some("Razer-X".to_string()),
                        copyright: Some("Copyright PLDreyer".to_string()),
                        ..Default::default()
                    }),
                )?;
                let quit_i = tauri::menu::MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
                let menu = tauri::menu::Menu::with_items(app, &[
                    &open_ui,
                    &sync_settings,
                    &separator,
                    &about,
                    &quit_i
                ])?;

                let tray = tauri::tray::TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&menu)
                    .build(app);

                Ok(())
            })
            .build(tauri::generate_context!())
            .expect("Failed to build Tauri application"),
    )

    /*
        app.run(|app, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        tauri::RunEvent::Reopen { has_visible_windows } => {
            // Create or show a window as necessary
        }
        _ => {}
    });
     */
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

