use crate::types::DpiStage;
use driver::UsbDriver;
use razer::DpiStage as RazerDpiStage;
use razer::{
    RazerReport, BACKLIGHT_LED, RAZER_BASILISK_V3_PRO_ID, RAZER_NEW_MOUSE_RECEIVER_WAIT_MAX_US,
    RAZER_USB_REPORT_LEN, RAZER_USB_VENDOR_ID, ZERO_LED,
};
use std::time::Duration;

unsafe fn get_data_for_razer_report(
    usb_handle: &mut driver::PlatformUsbDriver,
    index: u16,
    razer_report: &mut RazerReport,
) -> Result<Vec<u8>, String> {
    razer_report.finalize();
    let report_data = razer_report.to_hid_bytes();

    usb_handle.get_feature_report(
        report_data.as_slice(),
        index,
        Duration::from_micros(RAZER_NEW_MOUSE_RECEIVER_WAIT_MAX_US as u64),
        RAZER_USB_REPORT_LEN as u16,
    )
}

/// Lightweight check to see if the mouse is responsive.
/// Uses firmware version query and analyzes the response payload.
///
/// Returns:
/// - `true`: Mouse is powered ON and responding (status byte 0x02)
/// - `false`: Mouse is powered OFF (status byte 0x04) OR dongle is unplugged
///
/// This handles both:
/// 1. Wireless mouse power state (on/off while dongle stays plugged in)
/// 2. Physical dongle unplug (device not found)
pub unsafe fn is_mouse_alive() -> bool {
    match driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID) {
        Ok(mut usb_handle) => {
            let mut firmware_report = RazerReport::get_firmware_report();
            match get_data_for_razer_report(&mut usb_handle, 0x00, &mut firmware_report) {
                Ok(data) => {
                    // Status byte meanings:
                    // 0x02 = Command Successful (mouse is ON)
                    // 0x04 = Command No Response / Timeout (mouse is OFF)
                    let is_alive = data[0] == 0x02;

                    drop(usb_handle);
                    is_alive
                }
                Err(_) => {
                    drop(usb_handle);
                    false
                }
            }
        }
        Err(_) => false,
    }
}

pub unsafe fn get_battery_status() -> Result<u8, String> {
    let mut usb_handle =
        driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID)?;
    let res = get_battery_status_with_handle(&mut usb_handle);
    drop(usb_handle);
    res
}

pub unsafe fn get_battery_status_with_handle(
    usb_handle: &mut driver::PlatformUsbDriver,
) -> Result<u8, String> {
    let mut get_battery_report = RazerReport::get_battery_level_report();
    let data = get_data_for_razer_report(usb_handle, 0x00, &mut get_battery_report)?;
    let report = RazerReport::from_bytes(data.as_slice());
    let raw_battery_status = report.arguments[1];
    Ok((raw_battery_status as f32 / 255f32 * 100f32) as u8)
}

pub unsafe fn get_polling_rate() -> Result<u16, String> {
    let mut usb_handle =
        driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID)?;
    let res = get_polling_rate_with_handle(&mut usb_handle);
    drop(usb_handle);
    res
}

pub unsafe fn get_polling_rate_with_handle(
    usb_handle: &mut driver::PlatformUsbDriver,
) -> Result<u16, String> {
    let mut get_poll_rate_report = RazerReport::get_poll_rate_report();
    let data = get_data_for_razer_report(usb_handle, 0x00, &mut get_poll_rate_report)?;
    let report = RazerReport::from_bytes(data.as_slice());
    match report.arguments[0] {
        0x01 => Ok(1000),
        0x02 => Ok(500),
        0x08 => Ok(125),
        _ => Err(format!("Unknown polling rate: {}", report.arguments[0])),
    }
}

pub unsafe fn set_backlight(brightness: u8) -> Result<(), String> {
    let mut usb_handle =
        driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID)?;
    let res = set_backlight_with_handle(&mut usb_handle, brightness);
    drop(usb_handle);
    res
}

pub unsafe fn set_backlight_with_handle(
    usb_handle: &mut driver::PlatformUsbDriver,
    brightness: u8,
) -> Result<(), String> {
    let mut set_brightness_report = RazerReport::set_matrix_brightness_report(brightness);
    get_data_for_razer_report(usb_handle, 0x00, &mut set_brightness_report)?;
    let msg = format!("Backlight brightness successfully set to {}%", brightness);
    log::info!("{}", msg);
    println!("{}", msg);
    Ok(())
}

pub unsafe fn get_backlight() -> Result<u8, String> {
    let mut usb_handle =
        driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID)?;
    let res = get_backlight_with_handle(&mut usb_handle);
    drop(usb_handle);
    res
}

pub unsafe fn get_backlight_with_handle(
    usb_handle: &mut driver::PlatformUsbDriver,
) -> Result<u8, String> {
    let mut get_brightness_report = RazerReport::get_matrix_brightness_report();
    let data = get_data_for_razer_report(usb_handle, 0x00, &mut get_brightness_report)?;
    let report = RazerReport::from_bytes(data.as_slice());
    Ok(report.arguments[0])
}

pub unsafe fn set_polling_rate(polling_rate: u16) -> Result<(), String> {
    let mut usb_handle =
        driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID)?;
    let res = set_polling_rate_with_handle(&mut usb_handle, polling_rate);
    drop(usb_handle);
    res
}

pub unsafe fn set_polling_rate_with_handle(
    usb_handle: &mut driver::PlatformUsbDriver,
    polling_rate: u16,
) -> Result<(), String> {
    let mut set_poll_rate_report = RazerReport::set_poll_rate_report(polling_rate)?;
    get_data_for_razer_report(usb_handle, 0x00, &mut set_poll_rate_report)?;
    let msg = format!("Polling rate successfully set to {}Hz", polling_rate);
    log::info!("{}", msg);
    println!("{}", msg);
    Ok(())
}

pub unsafe fn get_dpi_xy() -> Result<(u16, u16), String> {
    let mut usb_handle =
        driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID)?;
    let res = get_dpi_xy_with_handle(&mut usb_handle);
    drop(usb_handle);
    res
}

pub unsafe fn get_dpi_xy_with_handle(
    usb_handle: &mut driver::PlatformUsbDriver,
) -> Result<(u16, u16), String> {
    let mut get_dpi_report = RazerReport::get_dpi_xy_report();
    let data = get_data_for_razer_report(usb_handle, 0x00, &mut get_dpi_report)?;
    let report = RazerReport::from_bytes(data.as_slice());
    let dpi_x = ((report.arguments[1] as u16) << 8) | (report.arguments[2] as u16 & 0xFF);
    let dpi_y = ((report.arguments[3] as u16) << 8) | (report.arguments[4] as u16 & 0xFF);
    Ok((dpi_x, dpi_y))
}

pub unsafe fn set_dpi_xy(dpi_x: u16, dpi_y: u16) -> Result<(), String> {
    let mut usb_handle =
        driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID)?;
    let res = set_dpi_xy_with_handle(&mut usb_handle, dpi_x, dpi_y);
    drop(usb_handle);
    res
}

pub unsafe fn set_dpi_xy_with_handle(
    usb_handle: &mut driver::PlatformUsbDriver,
    dpi_x: u16,
    dpi_y: u16,
) -> Result<(), String> {
    let mut set_dpi_report = RazerReport::set_dpi_xy_report(dpi_x, dpi_y);
    get_data_for_razer_report(usb_handle, 0x00, &mut set_dpi_report)?;
    let msg = format!("DPI successfully set to {}x{}", dpi_x, dpi_y);
    log::info!("{}", msg);
    println!("{}", msg);
    Ok(())
}

pub unsafe fn set_matrix_backlight_static(rgb: [u8; 3]) -> Result<(), String> {
    let mut usb_handle =
        driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID)?;
    let res = set_matrix_backlight_static_with_handle(&mut usb_handle, rgb);
    drop(usb_handle);
    res
}

pub unsafe fn set_matrix_backlight_static_with_handle(
    usb_handle: &mut driver::PlatformUsbDriver,
    rgb: [u8; 3],
) -> Result<(), String> {
    let mut set_static_report = RazerReport::set_matrix_effect_static_report(rgb, Some(ZERO_LED));
    get_data_for_razer_report(usb_handle, 0x00, &mut set_static_report)?;
    let msg = format!(
        "Matrix backlight successfully set to static RGB: [{}, {}, {}]",
        rgb[0], rgb[1], rgb[2]
    );
    log::info!("{}", msg);
    println!("{}", msg);
    Ok(())
}

pub unsafe fn get_led_rgb() -> Result<[u8; 3], String> {
    let mut usb_handle =
        driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID)?;
    let res = get_led_rgb_with_handle(&mut usb_handle);
    drop(usb_handle);
    res
}

pub unsafe fn get_led_rgb_with_handle(
    usb_handle: &mut driver::PlatformUsbDriver,
) -> Result<[u8; 3], String> {
    let mut get_led_report = RazerReport::get_led_rgb_report(Some(BACKLIGHT_LED));
    let data = get_data_for_razer_report(usb_handle, 0x00, &mut get_led_report)?;
    let report = RazerReport::from_bytes(data.as_slice());

    if report.arguments.len() < 3 {
        return Err("Invalid LED RGB data received".to_string());
    }

    Ok([
        report.arguments[0],
        report.arguments[1],
        report.arguments[2],
    ])
}

pub unsafe fn get_dpi_stages() -> Result<Vec<DpiStage>, String> {
    /*
    // Response format (hex):
    // 01    varstore
    // 02    active DPI stage
    // 04    number of stages = 4
    //
    // 01    first DPI stage
    // 03 20 first stage DPI X = 800
    // 03 20 first stage DPI Y = 800
    // 00 00 reserved
    //
    // 02    second DPI stage
    // 07 08 second stage DPI X = 1800
    // 07 08 second stage DPI Y = 1800
    // 00 00 reserved
    //
    // 03    third DPI stage
    // ...

        stages_count = response.arguments[2];

        buf[0] = response.arguments[1];

        count = 1;
        args = response.arguments + 4;
        for (i = 0; i < stages_count; i++) {
            // Check that we don't read past response.data_size
            if (args + 4 > response.arguments + response.data_size) {
                break;
            }

            memcpy(buf + count, args, 4);
            count += 4;
            args += 7;
        }

        return count;
     */
    let mut usb_handle =
        driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID)?;
    let res = get_dpi_stages_with_handle(&mut usb_handle);
    drop(usb_handle);
    res
}
pub unsafe fn get_dpi_stages_with_handle(
    usb_handle: &mut driver::PlatformUsbDriver,
) -> Result<Vec<DpiStage>, String> {
    let mut get_dpi_stages_report = RazerReport::get_dpi_stages_report();
    let data = get_data_for_razer_report(usb_handle, 0x00, &mut get_dpi_stages_report)?;
    let report = RazerReport::from_bytes(data.as_slice());

    if report.arguments.len() < 3 {
        return Err("Invalid DPI stages data received".to_string());
    }

    let mut data: Vec<DpiStage> = vec![];
    let mut args = &report.arguments[4..];
    let active_stage = report.arguments[1];
    let dpi_stages_count = report.arguments[2];

    for current_stage in 0..dpi_stages_count {
        if args.len() < 4 {
            break;
        }

        let dpi_x = ((args[0] as u16) << 8) | (args[1] as u16 & 0xFF);
        let dpi_y = ((args[2] as u16) << 8) | (args[3] as u16 & 0xFF);
        let active = (current_stage + 1) == active_stage;

        data.push(DpiStage {
            stage: current_stage + 1,
            dpi_x,
            dpi_y,
            active,
        });

        if args.len() >= 7 {
            args = &args[7..];
        } else {
            break;
        }
    }

    Ok(data)
}

pub unsafe fn set_dpi_stages(stages: Vec<DpiStage>) -> Result<(), String> {
    let mut usb_handle =
        driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID)?;
    let res = set_dpi_stages_with_handle(&mut usb_handle, stages);
    drop(usb_handle);
    res
}

pub unsafe fn set_dpi_stages_with_handle(
    usb_handle: &mut driver::PlatformUsbDriver,
    stages: Vec<DpiStage>,
) -> Result<(), String> {
    if stages.is_empty() {
        return Err("No DPI stages provided".to_string());
    }

    let active_dpi_stage = stages
        .iter()
        .find(|s| s.active)
        .map(|s| s.stage)
        .unwrap_or(1);
    let dpi_stages: Vec<RazerDpiStage> = stages
        .iter()
        .map(|dpi_stage| RazerDpiStage {
            stage: dpi_stage.stage,
            dpi_x: dpi_stage.dpi_x,
            dpi_y: dpi_stage.dpi_y,
        })
        .collect();

    let mut set_dpi_stages_report =
        RazerReport::set_dpi_stages_report(active_dpi_stage, dpi_stages);
    get_data_for_razer_report(usb_handle, 0x00, &mut set_dpi_stages_report)?;

    let msg = format!(
        "DPI stages successfully updated (Active Stage: {})",
        active_dpi_stage
    );
    log::info!("{}", msg);
    println!("{}", msg);
    Ok(())
}
