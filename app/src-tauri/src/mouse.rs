use std::time::Duration;
use driver::UsbDriver;
use razer::{RazerReport, BACKLIGHT_LED, RAZER_BASILISK_V3_PRO_ID, RAZER_NEW_MOUSE_RECEIVER_WAIT_MAX_US, RAZER_USB_VENDOR_ID, ZERO_LED};

unsafe fn get_data_for_razer_report(usb_handle: &mut driver::PlatformUsbDriver, index: u16, razer_report: &mut RazerReport) -> Result<Vec<u8>, String> {
    razer_report.finalize();
    let get_firmware_report_data = razer_report.to_hid_bytes();

    usb_handle.get_feature_report(
        get_firmware_report_data.as_slice(),
        index,
        Duration::from_micros(RAZER_NEW_MOUSE_RECEIVER_WAIT_MAX_US as u64),
        90
    )
}

pub unsafe fn get_battery_status() -> u8 {
    let mut usb_handle = driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID);
    let mut get_battery_report = RazerReport::get_battery_level_report();
    let battery_status = match get_data_for_razer_report(&mut usb_handle, 0x00, &mut get_battery_report) {
        Ok(data) => {
            let report = RazerReport::from_bytes(data.as_slice());
            let raw_battery_status = report.arguments[1];
            (raw_battery_status as f32 / 255f32 * 100f32) as u8
        },
        Err(e) => {
            eprintln!("Error getting battery status: {}", e);
            0
        }
    };
    drop(usb_handle);
    battery_status
}

pub unsafe fn get_polling_rate() -> u16 {
    let mut usb_handle = driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID);
    let mut get_poll_rate_report = RazerReport::get_poll_rate_report();
    let poll_rate = match get_data_for_razer_report(&mut usb_handle, 0x00, &mut get_poll_rate_report) {
        Ok(data) => {
            let report = RazerReport::from_bytes(data.as_slice());
            match report.arguments[0] {
                0x01 => 1000,
                0x02 => 500,
                0x08 => 125,
                _ => {
                    eprintln!("Unknown polling rate: {}", report.arguments[0]);
                    0
                }
            }
        },
        Err(e) => {
            eprintln!("Error getting polling rate: {}", e);
            0
        }
    };

    drop(usb_handle);
    poll_rate
}

pub unsafe fn set_backlight(brightness: u8) {
    let mut usb_handle = driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID);
    let mut set_brightness_report = RazerReport::set_matrix_brightness_report(brightness);
    match get_data_for_razer_report(&mut usb_handle, 0x00, &mut set_brightness_report) {
        Ok(_) => println!("Backlight set to {}%", brightness),
        Err(e) => eprintln!("Error setting backlight: {}", e),
    }
    drop(usb_handle);
}

pub unsafe fn get_backlight() -> Result<u8, String> {
    let mut usb_handle = driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID);
    let mut get_brightness_report = RazerReport::get_matrix_brightness_report();
    let result = get_data_for_razer_report(&mut usb_handle, 0x00, &mut get_brightness_report);
    drop(usb_handle);
    match result {
        Ok(data) => {
            let report = RazerReport::from_bytes(data.as_slice());
            Ok(report.arguments[0])
        },
        Err(e) => {
            eprintln!("Error getting backlight brightness: {}", e);
            Err(e)
        }
    }
}

pub unsafe fn set_polling_rate(polling_rate: u16) {
    let mut usb_handle = driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID);
    let mut set_poll_rate_report = RazerReport::set_poll_rate_report(polling_rate);
    match get_data_for_razer_report(&mut usb_handle, 0x00, &mut set_poll_rate_report) {
        Ok(_) => println!("Polling rate set to {}Hz", polling_rate),
        Err(e) => eprintln!("Error setting polling rate: {}", e),
    }
    drop(usb_handle);
}

pub unsafe fn get_dpi_xy() -> (u16, u16) {
    let mut usb_handle = driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID);
    let mut get_dpi_report = RazerReport::get_dpi_xy_report();
    match get_data_for_razer_report(&mut usb_handle, 0x00, &mut get_dpi_report) {
        Ok(data) => {
            drop(usb_handle);
            let report = RazerReport::from_bytes(data.as_slice());
            let dpi_x = ((report.arguments[1] as u16) << 8) | (report.arguments[2] as u16 & 0xFF);
            let dpi_y = ((report.arguments[3] as u16) << 8) | (report.arguments[4] as u16 & 0xFF);

            (dpi_x, dpi_y)
        },
        Err(e) => {
            drop(usb_handle);
            eprintln!("Error getting DPI: {}", e);
            (0, 0)
        }
    }
}

pub unsafe fn set_dpi_xy(dpi_x: u16, dpi_y: u16) {
    let mut usb_handle = driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID);
    let mut set_dpi_report = RazerReport::set_dpi_xy_report(dpi_x, dpi_y);
    
    match get_data_for_razer_report(&mut usb_handle, 0x00, &mut set_dpi_report) {
        Ok(_) => println!("DPI set to {}x{}", dpi_x, dpi_y),
        Err(e) => eprintln!("Error setting DPI: {}", e),
    }
    drop(usb_handle);
}

pub unsafe fn set_matrix_backlight_static(rgb: [u8; 3]) {
    let mut usb_handle = driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID);
    let mut set_static_report = RazerReport::set_matrix_effect_static_report(rgb, Some(ZERO_LED));
    match get_data_for_razer_report(&mut usb_handle, 0x00, &mut set_static_report) {
        Ok(_) => println!("Matrix backlight set to static"),
        Err(e) => eprintln!("Error setting matrix backlight: {}", e),
    }
    drop(usb_handle);
}

pub unsafe fn get_led_rgb() -> Result<[u8; 3], String> {
    let mut usb_handle = driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID);
    let mut get_led_report = RazerReport::get_led_rgb_report(Some(BACKLIGHT_LED));
    
    let result = get_data_for_razer_report(&mut usb_handle, 0x00, &mut get_led_report);
    drop(usb_handle);
    
    match result {
        Ok(data) => {
            let report = RazerReport::from_bytes(data.as_slice());
            println!("LED RGB: {:?}", report.arguments);
            println!("Report: {:?}", report);
            if report.arguments.len() < 3 {
                return Err("Invalid LED RGB data received".to_string());
            }
            
            let rgb = [
                report.arguments[0],
                report.arguments[1],
                report.arguments[2],
            ];
            
            println!("LED RGB: {:?}", rgb);
            Ok(rgb)
        },
        Err(e) => {
            eprintln!("Error getting LED RGB: {}", e);
            Err(e)
        }
    }
}