mod handler;
mod mouse;
mod types;

use std::sync::atomic::{AtomicBool, Ordering};
use tauri::path::BaseDirectory;
use tauri::Emitter;
use tauri::Manager;

struct IsQuitting(AtomicBool);
use driver::{PlatformUsbDriver, PreferencesDriver, UsbDriver};
use tauri_plugin_updater::UpdaterExt;
use handler::{
    apply_saved_settings, get_device_backlight_brightness, get_device_battery_status,
    get_device_charging_status, get_device_dpi_stages, get_device_information,
    get_device_led_rgb, get_saved_settings,
    get_target_os, save_settings, set_device_backlight_brightness, set_device_dpi,
    set_device_dpi_stages, set_device_matrix_backlight_static, set_device_polling_rate,
//    set_device_smart_wheel, 
    set_mouse_wheel_inverted,
};
use razer::{RAZER_BASILISK_V3_PRO_ID, RAZER_USB_VENDOR_ID};
use types::{DeviceCollection, DeviceInfo};

pub struct Application {
    pub app: tauri::App,
}

impl Application {
    pub fn new(app: tauri::App) -> Self {
        Application { app }
    }

    pub fn run(self) -> ! {
        self.app.run(|app, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                if !app.state::<IsQuitting>().0.load(Ordering::SeqCst) {
                    api.prevent_exit();
                }
            }
            tauri::RunEvent::Reopen {
                has_visible_windows,
                ..
            } => {
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
            .plugin(tauri_plugin_updater::Builder::new().build())
            .plugin(tauri_plugin_dialog::init())
            .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
                let _ = app.get_webview_window("main").map(|w| {
                    let _ = w.show();
                    let _ = w.set_focus();
                });
            }))
            .manage(IsQuitting(AtomicBool::new(false)))
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
//                set_device_smart_wheel,
                get_device_dpi_stages,
                set_device_dpi_stages,
                get_device_battery_status,
                get_device_charging_status,
                get_saved_settings,
                save_settings,
            ])
            .on_window_event(|window, event| match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    window.hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            })
            .setup(|app| {
                // Get windows
                let splashscreen = app.get_webview_window("splashscreen").unwrap();
                let main = app.get_webview_window("main").unwrap();
                
                let handle = app.handle().clone();
                let splash_clone = splashscreen.clone();
                let main_clone = main.clone();
                
                tauri::async_runtime::spawn(async move {
                    // Wait briefly for splashscreen to load
                    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                    
                    println!("Emitting loading-status: Checking for updates...");
                    let _ = handle.emit_to("splashscreen", "loading-status", "Checking for updates...");
                    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                    
                    match handle.updater().expect("failed to get updater").check().await {
                        Ok(Some(update)) => {
                            let msg = format!("Update available: {}", update.version);
                            log::info!("{}", msg);
                            println!("{}", msg);
                            let _ = handle.emit_to("splashscreen", "update-available", &update.version);

                            let mut downloaded = 0;
                            if let Err(e) = update
                                .download_and_install(
                                    |chunk_length, content_length| {
                                        downloaded += chunk_length;
                                        if let Some(total) = content_length {
                                            let progress = (downloaded as f64 / total as f64) * 100.0;
                                            let _ = handle.emit_to("splashscreen", "update-progress", progress);
                                        }
                                    },
                                    || {
                                        let _ = handle.emit_to("splashscreen", "update-status", "installing");
                                    },
                                )
                                .await
                            {
                                let err_msg = format!("Failed to download and install update: {}", e);
                                log::error!("{}", err_msg);
                                println!("{}", err_msg);
                                let _ = handle.emit_to("splashscreen", "update-error", err_msg);
                            } else {
                                let success_msg = "Update installed successfully";
                                log::info!("{}", success_msg);
                                println!("{}", success_msg);
                                let _ = handle.emit_to("splashscreen", "update-status", "finished");
                                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                                handle.restart();
                            }
                        }
                        Ok(None) => {
                            log::info!("No update available");
                            println!("No update available");
                        }
                        Err(e) => {
                            let err_msg = format!("Failed to check for updates: {}", e);
                            log::error!("{}", err_msg);
                            println!("{}", err_msg);
                        }
                    }
                    
                    println!("Emitting loading-status: Loading assets...");
                    let _ = handle.emit_to("splashscreen", "loading-status", "Loading assets...");
                    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                    
                    println!("Emitting loading-status: Applying default settings...");
                    let _ = handle.emit_to("splashscreen", "loading-status", "Applying default settings...");
                    
                    if unsafe { mouse::is_mouse_alive() } {
                        log::info!("Mouse already connected - applying saved settings");
                        println!("Mouse already connected - applying saved settings");
                        if let Ok(settings) = get_saved_settings(handle.clone()) {
                            unsafe {
                                apply_saved_settings(&settings);
                            }
                        }
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

                    println!("Emitting loading-status: Initializing...");
                    let _ = handle.emit_to("splashscreen", "loading-status", "Initializing...");
                    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                    
                    println!("Emitting initialization-complete");
                    let _ = handle.emit_to("splashscreen", "initialization-complete", ());
                    
                    // Small delay to ensure the message is seen before closing
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    
                    // Close splashscreen and show main window
                    let _ = splash_clone.close();
                    let _ = main_clone.show();
                    let _ = main_clone.set_focus();
                });

                let open_ui =
                    tauri::menu::MenuItem::with_id(app, "open_ui", "Open UI", true, None::<&str>)?;
                let battery_status = tauri::menu::MenuItem::with_id(
                    app,
                    "battery",
                    "Battery: --%",
                    true,
                    None::<&str>,
                )?;
                let check_updates = tauri::menu::MenuItem::with_id(
                    app,
                    "check_updates",
                    "Check for Updates",
                    true,
                    None::<&str>,
                )?;
                let separator = tauri::menu::PredefinedMenuItem::separator(app)?;
                let about = tauri::menu::PredefinedMenuItem::about(
                    app,
                    Some("About RazerX"),
                    Some(tauri::menu::AboutMetadata {
                        name: Some("RazerX".to_string()),
                        copyright: Some("Copyright PLDreyer".to_string()),
                        ..Default::default()
                    }),
                )?;
                let quit_i =
                    tauri::menu::MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
                let menu = tauri::menu::Menu::with_items(
                    app,
                    &[
                        &open_ui,
                        &battery_status,
                        &check_updates,
                        &separator,
                        &about,
                        &quit_i,
                    ],
                )?;

                let _tray = tauri::tray::TrayIconBuilder::new()
                    .icon(tauri::image::Image::from_path(
                        app.path()
                            .resolve("icons/TrayIcon.ico", BaseDirectory::Resource)?,
                    )?)
                    .menu(&menu)
                    .on_menu_event(|app, event| match event.id.as_ref() {
                        "open_ui" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "battery" => {
                            let app_handle = app.clone();
                            tauri::async_runtime::spawn(async move {
                                if let Ok(settings) = get_saved_settings(app_handle) {
                                    unsafe {
                                        apply_saved_settings(&settings);
                                    }
                                }
                            });
                        }
                        "check_updates" => {
                            let handle = app.clone();
                            tauri::async_runtime::spawn(async move {
                                use tauri_plugin_dialog::DialogExt;

                                match handle.updater().expect("failed to get updater").check().await {
                                    Ok(Some(update)) => {
                                        let response = handle
                                            .dialog()
                                            .message(format!(
                                                "A new version ({}) is available. Would you like to install it now?",
                                                update.version
                                            ))
                                            .title("Update Available")
                                            .buttons(tauri_plugin_dialog::MessageDialogButtons::YesNo)
                                            .blocking_show();

                                        if response {
                                            let _ = handle.emit("update-available", &update.version);
                                            let mut downloaded = 0;
                                            if let Err(e) = update
                                                .download_and_install(
                                                    |chunk_length, content_length| {
                                                        downloaded += chunk_length;
                                                        if let Some(total) = content_length {
                                                            let progress =
                                                                (downloaded as f64 / total as f64) * 100.0;
                                                            let _ = handle
                                                                .emit("update-progress", progress);
                                                        }
                                                    },
                                                    || {
                                                        let _ = handle
                                                            .emit("update-status", "installing");
                                                    },
                                                )
                                                .await
                                            {
                                                let err_msg = format!(
                                                    "Failed to download and install update: {}",
                                                    e
                                                );
                                                let _ = handle.emit("update-error", err_msg);
                                            } else {
                                                let _ = handle.emit("update-status", "finished");
                                                
                                                // Wait 3 seconds to let the user see the "finished" state in the UI
                                                let handle_clone = handle.clone();
                                                tauri::async_runtime::spawn(async move {
                                                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                                                    handle_clone.restart();
                                                });
                                            }
                                        }
                                    }
                                    Ok(None) => {
                                        handle
                                            .dialog()
                                            .message("You are running the latest version.")
                                            .title("No Updates Found")
                                            .kind(tauri_plugin_dialog::MessageDialogKind::Info)
                                            .show(|_| {});
                                    }
                                    Err(e) => {
                                        handle
                                            .dialog()
                                            .message(format!("Failed to check for updates: {}", e))
                                            .title("Update Error")
                                            .kind(tauri_plugin_dialog::MessageDialogKind::Error)
                                            .show(|_| {});
                                    }
                                }
                            });
                        }
                        "quit" => {
                            app.state::<IsQuitting>().0.store(true, Ordering::SeqCst);
                            app.exit(0);
                        }
                        _ => {}
                    })
                    .build(app)?;

                let battery_status_c = battery_status.clone();
                let app_handle = app.handle().clone();
                _tray.on_tray_icon_event(move |_tray_handle, event| {
                    if let tauri::tray::TrayIconEvent::Click { .. } = event {
                        let battery_status_item = battery_status_c.clone();
                        let handle = app_handle.clone();
                        tauri::async_runtime::spawn(async move {
                            match get_device_battery_status() {
                                Ok(level) => {
                                    let _ = battery_status_item.set_text(format!("Battery: {}%", level));
                                }
                                Err(_) => {
                                    let _ = battery_status_item.set_text("Battery: Unknown");
                                }
                            }
                        });
                    }
                });

                // Initial battery status update
                let battery_status_c = battery_status.clone();
                tauri::async_runtime::spawn(async move {
                    match get_device_battery_status() {
                        Ok(level) => {
                            let _ = battery_status_c.set_text(format!("Battery: {}%", level));
                        }
                        Err(_) => {
                            let _ = battery_status_c.set_text("Battery: Unknown");
                        }
                    }
                });

                let app_handle = app.handle().clone();
                PlatformUsbDriver::on_device_connected(
                    RAZER_USB_VENDOR_ID,
                    RAZER_BASILISK_V3_PRO_ID,
                    move |_device| {
                        log::info!("USB dongle connected - applying saved settings");
                        println!("USB dongle connected - applying saved settings");
                        let handle = app_handle.clone();
                        tauri::async_runtime::spawn(async move {
                            if let Ok(settings) = get_saved_settings(handle) {
                                unsafe {
                                    apply_saved_settings(&settings);
                                }
                            }
                        });
                    },
                )
                .map_err(|e| e.to_string())
                .expect("Failed to register connection hook");

                PlatformUsbDriver::on_device_disconnected(
                    RAZER_USB_VENDOR_ID,
                    RAZER_BASILISK_V3_PRO_ID,
                    |_device| {
                        log::info!("USB dongle disconnected - reverting trackpad settings");
                        println!("USB dongle disconnected - reverting trackpad settings");
                        let _ =
                            driver::PlatformPreferencesDriver::set_mouse_wheel_inverted(true);
                    },
                )
                .map_err(|e| e.to_string())
                .expect("Failed to register disconnection hook");

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
                                println!("Mouse powered ON - applying saved settings");
                                let handle = app_handle.clone();
                                tauri::async_runtime::spawn(async move {
                                    if let Ok(settings) = get_saved_settings(handle) {
                                        unsafe {
                                            apply_saved_settings(&settings);
                                        }
                                    }
                                });
                            } else {
                                log::info!("Mouse powered OFF - reverting trackpad settings");
                                println!("Mouse powered OFF - reverting trackpad settings");
                                let _ = driver::PlatformPreferencesDriver::set_mouse_wheel_inverted(
                                    true,
                                );
                            }
                        }
                    }
                });



                Ok(())
            })
            .build(tauri::generate_context!())
            .expect("Failed to build Tauri application"),
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = create_app();

    let path = app
        .app
        .path()
        .resolve("supported_devices", BaseDirectory::Resource)
        .unwrap();
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

    app.app.manage(DeviceCollection {
        devices: device_collection,
    });

    app.run();
}
