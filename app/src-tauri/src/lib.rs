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
    set_device_smart_wheel,
    get_device_backlight_brightness,
    set_device_dpi,
    get_device_dpi_stages,
    set_device_dpi_stages,
    get_device_battery_status,
    apply_saved_settings,
    get_saved_settings,
    save_settings,
};
use types::{DeviceCollection, DeviceInfo};
use driver::{PlatformUsbDriver, UsbDriver, PreferencesDriver};
use razer::{RAZER_BASILISK_V3_PRO_ID, RAZER_USB_VENDOR_ID};

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
                set_device_smart_wheel,
                get_device_dpi_stages,
                set_device_dpi_stages,
                get_device_battery_status,
                get_saved_settings,
                save_settings,])
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

                let _tray = tauri::tray::TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&menu)
                    .on_menu_event(|app, event| {
                        match event.id.as_ref() {
                            "open_ui" => {
                                if let Some(window) = app.get_webview_window("main") {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                            "sync" => {
                                let app_handle = app.clone();
                                tauri::async_runtime::spawn(async move {
                                    if let Ok(settings) = get_saved_settings(app_handle) {
                                        unsafe { apply_saved_settings(&settings); }
                                    }
                                });
                            }
                            "quit" => {
                                app.exit(0);
                            }
                            _ => {}
                        }
                    })
                    .build(app);

                // Register Hotplug Hooks
                let app_handle = app.handle().clone();
                unsafe {
                    PlatformUsbDriver::on_device_connected(
                        RAZER_USB_VENDOR_ID, 
                        RAZER_BASILISK_V3_PRO_ID, 
                        move |_device| {
                            log::info!("USB dongle connected - applying saved settings");
                            let handle = app_handle.clone();
                            tauri::async_runtime::spawn(async move {
                                if let Ok(settings) = get_saved_settings(handle) {
                                    unsafe { apply_saved_settings(&settings); }
                                }
                            });
                        }
                    ).expect("Failed to register connection hook");

                    PlatformUsbDriver::on_device_disconnected(
                        RAZER_USB_VENDOR_ID, 
                        RAZER_BASILISK_V3_PRO_ID, 
                        |_device| {
                            log::info!("USB dongle disconnected - reverting trackpad settings");
                            let _ = driver::PlatformPreferencesDriver::set_mouse_wheel_inverted(true);
                        }
                    ).expect("Failed to register disconnection hook");
                }
                
                // Start polling thread to detect wireless mouse power state changes
                let app_handle = app.handle().clone();
                std::thread::spawn(move || {
                    use std::sync::atomic::{AtomicBool, Ordering};
                    use std::time::Duration;
                    
                    static LAST_STATE: AtomicBool = AtomicBool::new(false);
                    
                    loop {
                        std::thread::sleep(Duration::from_secs(2));
                        
                        let is_alive = unsafe { mouse::is_mouse_alive() };
                        let last_state = LAST_STATE.load(Ordering::Relaxed);
                        
                        if is_alive != last_state {
                            LAST_STATE.store(is_alive, Ordering::Relaxed);
                            
                            if is_alive {
                                log::info!("Mouse powered ON - applying saved settings");
                                let handle = app_handle.clone();
                                tauri::async_runtime::spawn(async move {
                                    if let Ok(settings) = get_saved_settings(handle) {
                                        unsafe { apply_saved_settings(&settings); }
                                    }
                                });
                            } else {
                                log::info!("Mouse powered OFF - reverting trackpad settings");
                                let _ = driver::PlatformPreferencesDriver::set_mouse_wheel_inverted(true);
                            }
                        }
                    }
                });

                // Initial apply if mouse is already connected
                let app_handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    if unsafe { mouse::is_mouse_alive() } {
                        log::info!("Mouse already connected - applying saved settings");
                        if let Ok(settings) = get_saved_settings(app_handle) {
                            unsafe { apply_saved_settings(&settings); }
                        }
                    }
                });

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

