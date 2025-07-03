#![allow(unused)]
use std::{thread, time::Duration};
use std::sync::{Arc, Mutex};
use razer::{RazerReport, BACKLIGHT_LED, LOGO_LED, RAZER_BASILISK_V3_PRO_ID, RAZER_NEW_MOUSE_RECEIVER_WAIT_MAX_US, RAZER_USB_VENDOR_ID};
use driver::{UsbDriver};
use gui::Gui;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    service: Option<Service>,
}

#[derive(Subcommand, Debug, Clone)]
enum Service {
    Install,
    Uninstall,
}

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

fn main() -> ! {
    unsafe {
        let args = Args::parse();
        println!("Parsed arguments: {:?}", args);
        
        let usb_devices = driver::PlatformUsbDriver::list_devices();
        
        for device in &usb_devices {
            println!("Usb device: {:?}", device);
        }
        

        let razer_device = usb_devices.iter().find(| dev| {
            dev.product_id == RAZER_BASILISK_V3_PRO_ID as u32 && dev.vendor_id == RAZER_USB_VENDOR_ID as u32
        });

        if let Some(device) = razer_device {
            println!("Found Razer device: {:?}", device);
        } else {
            panic!("No Razer device found");
        }

        let get_battery_status = || {
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
        };

        let get_polling_rate = || {
            let mut usb_handle = driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID);
            let mut get_poll_rate_report = RazerReport::get_poll_rate_report();
            let poll_rate = match get_data_for_razer_report(&mut usb_handle, 0x00, &mut get_poll_rate_report) {
                Ok(data) => {
                    let report = RazerReport::from_bytes(data.as_slice());
                    println!("Report arguments for polling rate: {:?}", &report.arguments);
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
        };

        let set_backlight = |brightness: u8| {
            let mut usb_handle = driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID);
            let mut set_brightness_report = RazerReport::set_matrix_brightness_report(brightness);
            match get_data_for_razer_report(&mut usb_handle, 0x00, &mut set_brightness_report) {
                Ok(_) => println!("Backlight set to {}%", brightness),
                Err(e) => eprintln!("Error setting backlight: {}", e),
            }
            drop(usb_handle);
        };

        let set_polling_rate = |rate: u16| {
            let mut usb_handle = driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID);
            let mut set_poll_rate_report = RazerReport::set_poll_rate_report(rate);
            match get_data_for_razer_report(&mut usb_handle, 0x00, &mut set_poll_rate_report) {
                Ok(_) => println!("Polling rate set to {}Hz", rate),
                Err(e) => eprintln!("Error setting polling rate: {}", e),
            }
            drop(usb_handle);
        };

        let get_dpi_xy = || {
            let mut usb_handle = driver::PlatformUsbDriver::new(RAZER_USB_VENDOR_ID, RAZER_BASILISK_V3_PRO_ID);
            let mut get_dpi_report = RazerReport::get_dpi_xy_report();
            match get_data_for_razer_report(&mut usb_handle, 0x00, &mut get_dpi_report) {
                Ok(data) => {
                    let report = RazerReport::from_bytes(data.as_slice());
                    let dpi_x = ((report.arguments[1] as u16) << 8) | (report.arguments[2] as u16 & 0xFF);
                    let dpi_y = ((report.arguments[3] as u16) << 8) | (report.arguments[4] as u16 & 0xFF);
                    
                    (dpi_x, dpi_y)
                },
                Err(e) => {
                    eprintln!("Error getting DPI: {}", e);
                    (0, 0)
                }
            }
        };

        let mut gui = Gui::new(get_battery_status, set_backlight, set_polling_rate, get_polling_rate, get_dpi_xy);

        gui.run();
    }
}