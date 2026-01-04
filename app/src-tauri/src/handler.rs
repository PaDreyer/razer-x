use driver::{PlatformUsbDriver, UsbDriver, PreferencesDriver, PlatformPreferencesDriver};
use razer::{RAZER_BASILISK_V3_PRO_ID, RAZER_USB_VENDOR_ID};
use crate::mouse::{
    get_battery_status, get_battery_status_with_handle,
    get_polling_rate, get_polling_rate_with_handle,
    set_backlight, set_backlight_with_handle,
    set_polling_rate, set_polling_rate_with_handle,
    get_dpi_xy, get_dpi_xy_with_handle,
    set_matrix_backlight_static, set_matrix_backlight_static_with_handle,
    set_dpi_xy, set_dpi_xy_with_handle,
    get_led_rgb, get_led_rgb_with_handle,
    get_backlight, get_backlight_with_handle,
    get_dpi_stages, get_dpi_stages_with_handle,
    set_dpi_stages,
    is_mouse_alive
};
use crate::types::DpiStage;
use log::{info, error};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DeviceInfo {
    battery_level: u8,
    polling_rate: u16,
    dpi_xy: [u16; 2],
    backlight_brightness: u8,
    backlight_color: [u8; 3],
    matrix_behaviour: String,
    target_os: String,
    smart_wheel_enabled: bool,
    mouse_wheel_inverted: bool,
    dpi_stages: Vec<DpiStage>
}

pub unsafe fn ensure_mouse_exists() -> bool {
    let device_list = PlatformUsbDriver::list_devices();

    let razer_device = device_list.iter().find(|dev| {
        dev.product_id == RAZER_BASILISK_V3_PRO_ID as u32 && dev.vendor_id == RAZER_USB_VENDOR_ID as u32
    });
    
    razer_device.is_some()
}

#[tauri::command]
pub fn get_device_information() -> Option<String> {
    unsafe {
        let mut usb_handle = match PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID) {
            Ok(h) => h,
            Err(_) => return None,
        };

        let battery_status = get_battery_status_with_handle(&mut usb_handle).unwrap_or(0);
        let polling_rate = get_polling_rate_with_handle(&mut usb_handle).unwrap_or(0);
        let (dpi_x, dpi_y) = get_dpi_xy_with_handle(&mut usb_handle).unwrap_or((0, 0));
        let backlight_brightness = get_backlight_with_handle(&mut usb_handle).unwrap_or(0);
        let backlight_color = get_led_rgb_with_handle(&mut usb_handle).unwrap_or([0, 0, 0]);

        let mouse_wheel_inverted = PlatformPreferencesDriver::is_mouse_wheel_inverted().unwrap_or(false);

        let dpi_stages = get_dpi_stages_with_handle(&mut usb_handle).unwrap_or_default();

        let matrix_behaviour = "static"; 
        let smart_wheel_enabled = false; 

        let target_os = get_target_os();

        drop(usb_handle);

        let device_info = DeviceInfo {
            battery_level: battery_status,
            polling_rate,
            dpi_xy: [dpi_x, dpi_y],
            backlight_brightness,
            backlight_color,
            matrix_behaviour: matrix_behaviour.to_string(),
            target_os,
            smart_wheel_enabled,
            mouse_wheel_inverted,
            dpi_stages,
        };

        Some(serde_json::to_string(&device_info).unwrap())
    }
}

#[tauri::command]
pub fn get_device_battery_status() -> Result<u8, String> {
    unsafe {
        get_battery_status()
    }
}

#[tauri::command]
pub fn set_device_dpi(dpi_x: u16, dpi_y: u16) -> Result<(), String> {
    unsafe {
        set_dpi_xy(dpi_x, dpi_y)
    }
}

#[tauri::command]
pub fn set_device_backlight_brightness(brightness: u8) -> Result<(), String> {
    unsafe {
        set_backlight(brightness)
    }
}

#[tauri::command]
pub fn get_device_backlight_brightness() -> Result<u8, String> {
    unsafe {
        get_backlight()
    }
}

#[tauri::command]
pub fn set_device_polling_rate(polling_rate: u16) -> Result<(), String> {
    unsafe {
        set_polling_rate(polling_rate)
    }
}

#[tauri::command]
pub fn set_device_matrix_backlight_static(r: u8, g: u8, b: u8) -> Result<(), String> {
    unsafe {
        set_matrix_backlight_static([r, g, b])
    }
}

#[tauri::command]
pub fn get_device_led_rgb() -> Result<[u8; 3], String> {
    unsafe {
        get_led_rgb()
    }
}

#[tauri::command]
pub fn get_device_dpi_stages() -> Result<Vec<DpiStage>, String> {
    unsafe {
        get_dpi_stages()
    }
}

#[tauri::command]
pub fn set_device_dpi_stages(stages: Vec<DpiStage>) -> Result<(), String> {
    unsafe {
        set_dpi_stages(stages)
    }
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
pub fn set_mouse_wheel_inverted(inverted: bool) {
    PlatformPreferencesDriver::set_mouse_wheel_inverted(inverted).unwrap()
}

pub unsafe fn apply_default_settings() {
    info!("Applying default settings to device...");
    
    let mut usb_handle = match PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID) {
        Ok(h) => h,
        Err(e) => {
            error!("Failed to open device for applying settings: {}", e);
            return;
        }
    };

    // Use placeholders for default settings.
    let default_dpi = 3200;
    let default_polling_rate = 1000;
    let default_rgb = [255, 255, 255]; // White

    let _ = set_dpi_xy_with_handle(&mut usb_handle, default_dpi, default_dpi);
    let _ = set_polling_rate_with_handle(&mut usb_handle, default_polling_rate);
    let _ = set_matrix_backlight_static_with_handle(&mut usb_handle, default_rgb);
    
    drop(usb_handle);

    // Ensure mouse wheel is NOT inverted for the gaming mouse
    set_mouse_wheel_inverted(false);
}
