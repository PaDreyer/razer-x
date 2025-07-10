use std::thread::sleep;
use driver::{PlatformUsbDriver, UsbDriver};
use razer::{RAZER_BASILISK_V3_PRO_ID, RAZER_USB_VENDOR_ID};
use crate::mouse::{get_battery_status, get_polling_rate, set_backlight, set_polling_rate, get_dpi_xy, set_matrix_backlight_static, set_dpi_xy, get_led_rgb, get_backlight};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct DeviceInfo {
    battery_status: u8,
    polling_rate: u16,
    dpi_x: u16,
    dpi_y: u16,
}

pub unsafe fn ensure_mouse_exists() -> bool {
    let device_list = PlatformUsbDriver::list_devices();

    let razer_device = device_list.iter().find(|dev| {
        dev.product_id == RAZER_BASILISK_V3_PRO_ID as u32 && dev.vendor_id == RAZER_USB_VENDOR_ID as u32
    });
    
    return razer_device.is_some();
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

        let device_info = DeviceInfo {
            battery_status,
            polling_rate,
            dpi_x,
            dpi_y,
        };

        Some(serde_json::to_string(&device_info).unwrap())
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
pub fn get_device_led_rgb() -> Option<[u8; 3]> {
    unsafe {
        if !ensure_mouse_exists() {
            return None;
        }
        
        match get_led_rgb() {
            Ok(rgb) => Some(rgb),
            Err(e) => {
                eprintln!("Error getting LED RGB: {}", e);
                None
            }
        }
    }
}