use std::thread::sleep;
use driver::{PlatformUsbDriver, UsbDriver, PreferencesDriver, PlatformPreferencesDriver};
use razer::{RAZER_BASILISK_V3_PRO_ID, RAZER_USB_VENDOR_ID};
use crate::mouse::{get_battery_status, get_polling_rate, set_backlight, set_polling_rate, get_dpi_xy, set_matrix_backlight_static, set_dpi_xy, get_led_rgb, get_backlight, get_dpi_stages, set_dpi_stages};
use crate::types::DpiStage;

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

# [tauri::command]
pub fn get_device_information() -> Option<String> {
    unsafe {
        if !ensure_mouse_exists() {
            return None;
        }

        let battery_status = get_battery_status();
        sleep(std::time::Duration::from_millis(100));
        let polling_rate = get_polling_rate();
        sleep(std::time::Duration::from_millis(100));
        let (dpi_x, dpi_y) = get_dpi_xy();
        sleep(std::time::Duration::from_millis(100));
        let backlight_brightness = get_backlight().unwrap();
        sleep(std::time::Duration::from_millis(100));
        let backlight_color = get_led_rgb().unwrap();
        sleep(std::time::Duration::from_millis(100));

        let mouse_wheel_inverted = PlatformPreferencesDriver::is_mouse_wheel_inverted().unwrap();
        sleep(std::time::Duration::from_millis(100));

        let dpi_stages = get_dpi_stages().unwrap();

        let matrix_behaviour = "static"; // Placeholder, as the actual behaviour is not implemented in this example
        let smart_wheel_enabled = false; // Placeholder, as the actual smart wheel state is not implemented in this example

        let target_os = get_target_os();

        let device_info = DeviceInfo {
            battery_level: battery_status,
            polling_rate,
            dpi_xy: [dpi_x, dpi_y],
            backlight_brightness, // kaputt
            backlight_color, // finde den befehl nicht
            matrix_behaviour: matrix_behaviour.to_string(), // glaube kaum dass es das gibt
            target_os,
            smart_wheel_enabled, // mal schauen ob es das gibt
            mouse_wheel_inverted,
            dpi_stages,
        };

        Some(serde_json::to_string(&device_info).unwrap())
    }
}

#[tauri::command]
pub fn get_device_battery_status() -> Result<u8, String> {
    unsafe {
        if !ensure_mouse_exists() {
            return Err(String::from("Unable to find device"));
        }

        return Ok(get_battery_status());
    }
}

#[tauri::command]
pub fn set_device_dpi(dpi_x: u16, dpi_y: u16) {
    unsafe {
        set_dpi_xy(dpi_x, dpi_y);
    }
}

#[tauri::command]
pub fn set_device_backlight_brightness(brightness: u8) {
    unsafe {
        set_backlight(brightness);
    }
}

#[tauri::command]
pub fn get_device_backlight_brightness() -> Result<u8, String> {
    unsafe {
        if !ensure_mouse_exists() {
            return Err(String::from("Unable to find device"));
        }
        
        match get_backlight() {
            Ok(brightness) => Ok(brightness),
            Err(e) => {
                eprintln!("Error getting backlight brightness: {}", e);
                Err(e)
            }
        }
    }
}

#[tauri::command]
pub fn set_device_polling_rate(polling_rate: u16) {
    unsafe {
        set_polling_rate(polling_rate);
    }
}

#[tauri::command]
pub fn set_device_matrix_backlight_static(r: u8, g: u8, b: u8) {
    unsafe {
        if !ensure_mouse_exists() {
            return;
        }

        set_matrix_backlight_static([r, g, b]);
    }
}

#[tauri::command]
pub fn get_device_led_rgb() -> Result<[u8; 3], String> {
    unsafe {
        if !ensure_mouse_exists() {
            return Err(String::from("Unable to find device"));
        }
        
        return get_led_rgb();
    }
}

#[tauri::command]
pub fn get_device_dpi_stages() -> Result<Vec<DpiStage>, String> {
    unsafe {
        if !ensure_mouse_exists() {
            return Err(String::from("Unable to find device"));
        }

        return get_dpi_stages();
    }
}

#[tauri::command]
pub fn set_device_dpi_stages(stages: Vec<DpiStage>) -> Result<(), String> {
    unsafe {
        if !ensure_mouse_exists() {
            return Err(String::from("Unable to find device"));
        }

        return set_dpi_stages(stages);
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
pub fn set_mouse_wheel_inverted(inverted: bool) -> () {
    PlatformPreferencesDriver::set_mouse_wheel_inverted(inverted).unwrap()
}
