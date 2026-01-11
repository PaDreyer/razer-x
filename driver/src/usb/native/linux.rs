// Neue Version des LinuxUsbDriver mit direkten libusb-Bindings (aus dem crate `bindings`)

use std::{thread, time::Duration};
use std::ffi::CStr;
use std::ptr;
use std::path::PathBuf;

use super::{UsbDriver, Device};
use crate::{DriverResult, DriverError};
use bindings::{libusb_context, libusb_get_device_list, libusb_get_device_descriptor, libusb_device_handle, libusb_open, libusb_claim_interface, libusb_free_device_list, libusb_control_transfer, libusb_release_interface, libusb_close, libusb_init, libusb_device, libusb_exit};

pub const LIBUSB_ENDPOINT_IN: u8 = 0x80;
pub const LIBUSB_ENDPOINT_OUT: u8 = 0x00;

pub const LIBUSB_REQUEST_TYPE_STANDARD: u8 = 0x00;
pub const LIBUSB_REQUEST_TYPE_CLASS: u8 = 0x20;
pub const LIBUSB_REQUEST_TYPE_VENDOR: u8 = 0x40;

pub const LIBUSB_RECIPIENT_DEVICE: u8 = 0x00;
pub const LIBUSB_RECIPIENT_INTERFACE: u8 = 0x01;
pub const LIBUSB_RECIPIENT_ENDPOINT: u8 = 0x02;
pub const LIBUSB_RECIPIENT_OTHER: u8 = 0x03;

pub struct LinuxUsbDriver {
    handle: *mut libusb_device_handle,
    vendor_id: u16,
    product_id: u16,
    interface_index: u8,
}

impl LinuxUsbDriver {
    unsafe fn find_and_open_device(vendor_id: u16, product_id: u16) -> Result<*mut libusb_device_handle, String> {
        let mut ctx: *mut libusb_context = ptr::null_mut();
        if libusb_init(&mut ctx) != 0 {
            return Err("Failed to init libusb".into());
        }

        let mut list: *mut *mut libusb_device = ptr::null_mut();
        let count = libusb_get_device_list(ctx, &mut list);
        if count < 0 {
            return Err("Failed to get device list".into());
        }

        for i in 0..count {
            let device = *list.offset(i as isize);
            let mut desc = std::mem::zeroed();
            if libusb_get_device_descriptor(device, &mut desc) != 0 {
                continue;
            }

            if desc.idVendor == vendor_id && desc.idProduct == product_id {
                let mut handle: *mut libusb_device_handle = ptr::null_mut();
                if libusb_open(device, &mut handle) == 0 {
                    libusb_claim_interface(handle, 0);
                    libusb_free_device_list(list, 1);
                    return Ok(handle);
                }
            }
        }

        libusb_free_device_list(list, 1);
        Err("Device not found".to_string())
    }
}

impl UsbDriver for LinuxUsbDriver {
    unsafe fn new(vendor_id: u16, product_id: u16) -> DriverResult<Self> {
        let handle = match Self::find_and_open_device(vendor_id, product_id) {
            Ok(h) => h,
            Err(e) => return Err(DriverError::DeviceNotFound(vendor_id, product_id)),
        };

        Ok(Self {
            handle,
            vendor_id,
            product_id,
            interface_index: 0,
        })
    }

    unsafe fn list_devices() -> Vec<Device> {
        let mut devices = vec![];
        let mut ctx: *mut libusb_context = ptr::null_mut();
        if libusb_init(&mut ctx) != 0 {
            return devices;
        }

        let mut list: *mut *mut libusb_device = ptr::null_mut();
        let count = libusb_get_device_list(ctx, &mut list);
        if count < 0 {
            return devices;
        }

        for i in 0..count {
            let device = *list.offset(i as isize);
            let mut desc = std::mem::zeroed();
            if libusb_get_device_descriptor(device, &mut desc) != 0 {
                continue;
            }

            let name = format!("{:04x}:{:04x}", desc.idVendor, desc.idProduct);
            devices.push(Device {
                name,
                vendor_id: desc.idVendor as u32,
                product_id: desc.idProduct as u32,
            });
        }

        libusb_free_device_list(list, 1);
        libusb_exit(ctx);

        devices
    }

    unsafe fn send_control_msg(
        &mut self,
        request: u8,
        value: u16,
        index: u16,
        data: &[u8],
        min_wait: Duration,
    ) -> DriverResult<()> {
        let bm_request_type = 0x21;
        let timeout = 1000;

        let transferred = libusb_control_transfer(
            self.handle,
            bm_request_type,
            request,
            value,
            index,
            data.as_ptr() as *mut u8,
            data.len() as u16,
            timeout,
        );

        if transferred < 0 {
            return Err(DriverError::UsbError(format!("Control transfer failed: code {}", transferred)));
        }

        if transferred != data.len() as i32 {
            return Err(DriverError::IncompleteTransfer);
        }

        thread::sleep(min_wait);
        Ok(())
    }

    unsafe fn get_feature_report(
        &mut self,
        data: &[u8],
        index: u16,
        min_wait: Duration,
        response_length: u16,
    ) -> DriverResult<Vec<u8>> {
        let _ = self.send_control_msg(0x09, 0x300, index, data, min_wait);

        let bm_request_type = 0xA1;
        let mut buffer = vec![0u8; response_length as usize];

        let read = libusb_control_transfer(
            self.handle,
            bm_request_type,
            0x01, // HID_GET_REPORT
            0x300,
            index,
            buffer.as_mut_ptr(),
            response_length,
            10000,
        );

        if read < 0 {
            return Err(DriverError::UsbError(format!("read_control failed: code {}", read)));
        }

        Ok(buffer[..read as usize].to_vec())
    }

    unsafe fn close(&mut self) -> DriverResult<()> {
        let rc = libusb_release_interface(self.handle, self.interface_index as i32);
        if rc != 0 {
            return Err(DriverError::UsbError(format!("Failed to release interface: code {}", rc)));
        }
        libusb_close(self.handle);
        Ok(())
    }
    
    fn on_device_connected<F>(_vendor_id: u16, _product_id: u16, _callback: F) -> DriverResult<()>
    where
        F: FnMut(&Device) + Send + 'static,
    {
         Err(DriverError::NotImplemented("Linux hotplug not implemented yet".into()))
    }

    fn on_device_disconnected<F>(_vendor_id: u16, _product_id: u16, _callback: F) -> DriverResult<()>
    where
        F: FnMut(&Device) + Send + 'static,
    {
         Err(DriverError::NotImplemented("Linux hotplug not implemented yet".into()))
    }

    fn on_state_changed<F>(&mut self, _callback: F) -> DriverResult<()>
    where
        F: FnMut(&Device, &mut c_void) + Send + 'static,
    {
         Err(DriverError::NotImplemented("Linux state changed not implemented yet".into()))
    }
}
