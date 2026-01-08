use crate::mouse::{
    get_backlight, get_battery_status, get_battery_status_with_handle, get_dpi_stages,
    get_led_rgb, set_backlight, set_backlight_with_handle, set_dpi_stages,
    set_dpi_stages_with_handle, set_dpi_xy, set_dpi_xy_with_handle, set_matrix_backlight_static,
    set_matrix_backlight_static_with_handle, set_polling_rate, set_polling_rate_with_handle,
};
use driver::settings::{DpiStage, MouseSettings};
use driver::{PlatformPreferencesDriver, PlatformUsbDriver, PreferencesDriver, UsbDriver};
use log::{error, info};
use razer::{RAZER_BASILISK_V3_PRO_ID, RAZER_USB_VENDOR_ID};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct RgbColor {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DeviceInfo {
    battery_level: u8,
    polling_rate: u16,
    dpi_xy: [u16; 2],
    backlight_brightness: u8,
    backlight_color: RgbColor,
    matrix_behavior: String,
    target_os: String,
    smart_wheel_enabled: bool,
    mouse_wheel_inverted: bool,
    dpi_stages: Vec<DpiStage>,
}

pub unsafe fn ensure_mouse_exists() -> bool {
    let device_list = PlatformUsbDriver::list_devices();

    let razer_device = device_list.iter().find(|dev| {
        dev.product_id == RAZER_BASILISK_V3_PRO_ID as u32
            && dev.vendor_id == RAZER_USB_VENDOR_ID as u32
    });

    razer_device.is_some()
}

#[tauri::command]
pub fn get_device_information(app: AppHandle) -> Option<String> {
    unsafe {
        let mut usb_handle =
            match PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID) {
                Ok(h) => h,
                Err(_) => return None,
            };

        let battery_status = get_battery_status_with_handle(&mut usb_handle).unwrap_or(0);

        // Load saved settings to ensure UI is in sync with persistent state
        let settings = get_saved_settings(app).unwrap_or_default();

        let target_os = get_target_os();

        drop(usb_handle);

        // Check if natural scrolling (mouse wheel inversion) state is in sync with OS preference
        let current_os_inverted = PlatformPreferencesDriver::is_mouse_wheel_inverted()
            .unwrap_or(settings.scroll_inverted);
        if current_os_inverted != settings.scroll_inverted {
            log::info!(
                "Syncing natural scrolling state: OS={} -> Saved={}",
                current_os_inverted,
                settings.scroll_inverted
            );
            let _ = PlatformPreferencesDriver::set_mouse_wheel_inverted(settings.scroll_inverted);
        }

        let device_info = DeviceInfo {
            battery_level: battery_status,
            polling_rate: settings.polling_rate,
            dpi_xy: [settings.dpi_x, settings.dpi_y],
            backlight_brightness: settings.brightness,
            backlight_color: RgbColor {
                r: settings.rgb_color[0],
                g: settings.rgb_color[1],
                b: settings.rgb_color[2],
            },
            matrix_behavior: "static".to_string(),
            target_os,
            smart_wheel_enabled: settings.smart_wheel_enabled,
            mouse_wheel_inverted: settings.scroll_inverted,
            dpi_stages: settings.dpi_stages,
        };

        Some(serde_json::to_string(&device_info).unwrap())
    }
}

#[tauri::command]
pub fn set_device_smart_wheel(app: AppHandle, enabled: bool) -> Result<(), String> {
    // Note: Implementation of hardware protocol for smart wheel is pending
    // For now, we persist the setting so the UI remains consistent
    let res = update_settings(app, |s| s.smart_wheel_enabled = enabled);
    if res.is_ok() {
        let msg = format!(
            "Smart Wheel setting successfully {} (Persistence only)",
            if enabled { "enabled" } else { "disabled" }
        );
        log::info!("{}", msg);
        println!("{}", msg);
    }
    res
}

#[tauri::command]
pub fn get_device_battery_status() -> Result<u8, String> {
    unsafe { get_battery_status() }
}

#[tauri::command]
pub fn set_device_dpi(app: AppHandle, dpi_x: u16, dpi_y: u16) -> Result<(), String> {
    unsafe {
        set_dpi_xy(dpi_x, dpi_y)?;
    }
    update_settings(app, |s| {
        s.dpi_x = dpi_x;
        s.dpi_y = dpi_y;
    })
}

#[tauri::command]
pub fn set_device_backlight_brightness(app: AppHandle, brightness: u8) -> Result<(), String> {
    unsafe {
        set_backlight(brightness)?;
    }
    update_settings(app, |s| s.brightness = brightness)
}

#[tauri::command]
pub fn get_device_backlight_brightness() -> Result<u8, String> {
    unsafe { get_backlight() }
}

#[tauri::command]
pub fn set_device_polling_rate(app: AppHandle, polling_rate: u16) -> Result<(), String> {
    unsafe {
        set_polling_rate(polling_rate)?;
    }
    update_settings(app, |s| s.polling_rate = polling_rate)
}

#[tauri::command]
pub fn set_device_matrix_backlight_static(
    app: AppHandle,
    r: u8,
    g: u8,
    b: u8,
) -> Result<(), String> {
    unsafe {
        set_matrix_backlight_static([r, g, b])?;
    }
    update_settings(app, |s| s.rgb_color = [r, g, b])
}

#[tauri::command]
pub fn get_device_led_rgb() -> Result<[u8; 3], String> {
    unsafe { get_led_rgb() }
}

#[tauri::command]
pub fn get_device_dpi_stages() -> Result<Vec<DpiStage>, String> {
    unsafe { get_dpi_stages() }
}

#[tauri::command]
pub fn set_device_dpi_stages(app: AppHandle, stages: Vec<DpiStage>) -> Result<(), String> {
    unsafe {
        set_dpi_stages(stages.clone())?;
    }
    update_settings(app, |s| s.dpi_stages = stages)
}

#[tauri::command]
pub fn get_target_os() -> String {
    #[cfg(target_os = "windows")]
    return String::from("windows");

    #[cfg(target_os = "linux")]
    return String::from("linux");

    #[cfg(target_os = "macos")]
    return String::from("macos");

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    return String::from("unknown");
}

#[tauri::command]
pub fn set_mouse_wheel_inverted(app: AppHandle, inverted: bool) -> Result<(), String> {
    PlatformPreferencesDriver::set_mouse_wheel_inverted(inverted)?;
    let res = update_settings(app, |s| s.scroll_inverted = inverted);
    if res.is_ok() {
        let msg = format!(
            "Mouse wheel inversion successfully {} (System preference applied)",
            if inverted { "enabled" } else { "disabled" }
        );
        log::info!("{}", msg);
        println!("{}", msg);
    }
    res
}

fn update_settings<F>(app: AppHandle, updater: F) -> Result<(), String>
where
    F: FnOnce(&mut MouseSettings),
{
    let path = get_settings_path(&app)?;
    let mut settings = MouseSettings::load(&path).map_err(|e| e.to_string())?;
    updater(&mut settings);
    settings.save(&path).map_err(|e| e.to_string())
}

fn get_settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map(|path| path.join("settings.json"))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_saved_settings(app: AppHandle) -> Result<MouseSettings, String> {
    let path = get_settings_path(&app)?;
    MouseSettings::load(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings: MouseSettings) -> Result<(), String> {
    let path = get_settings_path(&app)?;
    settings.save(&path).map_err(|e| e.to_string())
}

pub unsafe fn apply_saved_settings(settings: &MouseSettings) {
    info!("Applying saved settings to device: {:?}", settings);
    println!("Applying saved settings to device: {:?}", settings);

    let mut usb_handle = match PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID)
    {
        Ok(h) => h,
        Err(e) => {
            error!("Failed to open device for applying settings: {}", e);
            return;
        }
    };

    let _ = set_dpi_xy_with_handle(&mut usb_handle, settings.dpi_x, settings.dpi_y);
    let _ = set_dpi_stages_with_handle(&mut usb_handle, settings.dpi_stages.clone());
    let _ = set_polling_rate_with_handle(&mut usb_handle, settings.polling_rate);
    let _ = set_matrix_backlight_static_with_handle(&mut usb_handle, settings.rgb_color);
    let _ = set_backlight_with_handle(&mut usb_handle, settings.brightness);

    drop(usb_handle);

    // Ensure mouse wheel inversion is applied if supported/requested
    let _ = PlatformPreferencesDriver::set_mouse_wheel_inverted(settings.scroll_inverted)
        .map_err(|e| e.to_string());
}

pub unsafe fn apply_default_settings() {
    info!("Applying default settings...");
    println!("Applying default settings...");
    apply_saved_settings(&MouseSettings::default());
}
