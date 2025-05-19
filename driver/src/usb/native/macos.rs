use std::ffi::CStr;
use std::os::raw::{c_void, c_char};
use std::{ptr, thread};
use std::time::Duration;
use super::{Device, UsbDriver};
use bindings::{kIOReturnSuccess, IOServiceMatching, IOServiceGetMatchingServices, IOIteratorNext, IOObjectRelease, IOCFPlugInInterface, IOCreatePlugInInterfaceForService, CFUUIDGetUUIDBytes, kIOMasterPortDefault, io_iterator_t, get_plugin_uuid, get_usb_device_uuid, get_usb_device_interface_uuid, io_registry_entry_t, IORegistryEntryCreateCFProperty, kCFStringEncodingUTF8, CFStringGetCString, CFString, TCFType, IOUSBDeviceInterface, IOUSBDevRequest, IOReturn, CFNumberGetValue, kCFNumberSInt32Type};
use log::{log, info, debug, warn, error};

unsafe fn get_string_property(entry: io_registry_entry_t, key: &str) -> Option<String> {
    let cf_key = CFString::new(key);
    let raw = IORegistryEntryCreateCFProperty(
        entry,
        cf_key.as_concrete_TypeRef(),
        ptr::null_mut(),
        0,
    );
    if raw.is_null() {
        return None;
    }

    let mut buf = [0 as c_char; 256];
    let success = CFStringGetCString(
        raw as _,
        buf.as_mut_ptr(),
        buf.len() as isize,
        kCFStringEncodingUTF8,
    );
    if success == 0 {
        return None;
    }

    Some(CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned())
}

unsafe fn get_int_property(entry: io_registry_entry_t, key: &str) -> Option<u32> {
    let cf_key = CFString::new(key);
    let raw = IORegistryEntryCreateCFProperty(
        entry,
        cf_key.as_concrete_TypeRef(),
        ptr::null_mut(),
        0,
    );
    if raw.is_null() {
        return None;
    }

    let mut val: i32 = 0;
    let success = CFNumberGetValue(raw as _, kCFNumberSInt32Type, &mut val as *mut _ as *mut _);
    if success != false {
        Some(val as u32)
    } else {
        None
    }
}


pub struct MacOsUsbDriver {
    vendor_id: String,
    product_id: String,
    device: *mut *mut IOUSBDeviceInterface,
}

impl Drop for MacOsUsbDriver {
    fn drop(&mut self) {
        if !self.device.is_null() {
            unsafe { self.close().unwrap(); };
        }
    }
}

impl UsbDriver for MacOsUsbDriver {
    unsafe fn new(vendor_id: &str, product_id: &str) -> Self {
        let matching_dict = IOServiceMatching(b"IOUSBDevice\0".as_ptr() as *const i8);
        if matching_dict.is_null() {
            panic!("IOServiceMatching failed");
        }

        let mut iter: io_iterator_t = 0;
        let result = IOServiceGetMatchingServices(kIOMasterPortDefault, matching_dict, &mut iter);
        if result != kIOReturnSuccess {
            panic!("IOServiceGetMatchingServices failed");
        }

        let mut device: Option<*mut *mut IOUSBDeviceInterface> = None;
        loop {
            let usb_device = IOIteratorNext(iter);

            if usb_device == 0 {
                println!("No more devices found.");
                device = None;
            }

            let usb_device_name = get_string_property(usb_device, "USB Product Name").unwrap();
            println!("Found USB device: {}", usb_device_name);

            let vendor_id = get_string_property(usb_device, "idVendor").unwrap();
            println!("Vendor ID: {}", vendor_id);

            let mut plugin_interface: *mut IOCFPlugInInterface = ptr::null_mut();
            let mut plugin_interface_ptr: *mut *mut IOCFPlugInInterface = &mut plugin_interface;
            let mut score: i32 = 0;

            let kr = IOCreatePlugInInterfaceForService(
                usb_device,
                get_usb_device_uuid(),
                get_plugin_uuid(),
                &mut plugin_interface_ptr,
                &mut score,
            );

            IOObjectRelease(usb_device);

            if kr != kIOReturnSuccess || (plugin_interface_ptr as i32) == 0x0 {
                println!("IOCreatePlugInInterfaceForService failed");
                continue;
            } else {
                println!("IOCreatePlugInInterfaceForService succeeded");
            }


            let mut iface_ptr: *mut IOUSBDeviceInterface = ptr::null_mut();
            let mut iface_ptr_ptr: *mut *mut IOUSBDeviceInterface = &mut iface_ptr;
            let mut iface_ptr_ptr_ptr: *mut *mut *mut IOUSBDeviceInterface = &mut iface_ptr_ptr;

            let hresult = (**plugin_interface_ptr).QueryInterface.unwrap()(
                plugin_interface_ptr as *mut c_void,
                CFUUIDGetUUIDBytes(get_usb_device_interface_uuid()),
                iface_ptr_ptr_ptr as *mut *mut c_void,
            );

            (**plugin_interface_ptr).Release.unwrap()(plugin_interface_ptr as *mut c_void);

            if hresult != 0 || iface_ptr_ptr.is_null() {
                println!("QueryInterface failed");
                continue;
            }

            if (**iface_ptr_ptr).USBDeviceOpen.is_none() {
                println!("USBDeviceOpen function pointer is null");
                (**iface_ptr_ptr).Release.unwrap()(iface_ptr_ptr as *mut c_void);
                continue;
            }

            let open_result = (**iface_ptr_ptr).USBDeviceOpen.unwrap()(iface_ptr_ptr as *mut c_void);
            println!("USBDeviceOpen result: {:#x}", open_result);

            if open_result != kIOReturnSuccess {
                println!("Unable to open USB device: {:08x}", open_result);
                (**iface_ptr_ptr).Release.unwrap()(iface_ptr_ptr as *mut c_void);
                continue;
            }

            IOObjectRelease(iter);
            device = Some(iface_ptr_ptr);
            break;
        }

        if device.is_none() {
            panic!("No device found");
        }

        Self {
            vendor_id: String::from(vendor_id),
            product_id: String::from(product_id),
            device: device.unwrap(),
        }
    }

    unsafe fn list_devices() -> Vec<Device> {
        let mut devices: Vec<Device> = vec![];

        let matching_dict = IOServiceMatching(b"IOUSBDevice\0".as_ptr() as *const i8);
        if matching_dict.is_null() {
            panic!("IOServiceMatching failed");
        }

        let mut iter: io_iterator_t = 0;
        let result = IOServiceGetMatchingServices(kIOMasterPortDefault, matching_dict, &mut iter);
        if result != kIOReturnSuccess {
            panic!("IOServiceGetMatchingServices failed");
        }

        loop {
            let usb_device = IOIteratorNext(iter);

            if usb_device == 0 {
                println!("No more devices found.");
                break;
            }

            let usb_device_name = get_string_property(usb_device, "USB Product Name").unwrap();
            println!("Found USB device: {}", usb_device_name);

            let vendor_id = get_int_property(usb_device, "idVendor").unwrap();
            println!("Vendor ID: {}", vendor_id);

            let product_id = get_int_property(usb_device, "idProduct").unwrap();
            println!("Product ID: {}", product_id);

            devices.push(Device {
                name: usb_device_name,
                vendor_id,
                product_id,
            });

            IOObjectRelease(usb_device);
        }

        IOObjectRelease(iter);

        devices
    }

    unsafe fn send_control_msg(&mut self, request: u8, value: u16, index: u16, data: &[u8], min_wait: Duration) -> Result<IOReturn, String> {
        if self.device.is_null() {
            return Err("Device is null".to_string());
        }

        let mut buffer = data.to_vec();
        let mut req = IOUSBDevRequest {
            bmRequestType: 0x21, // USB_TYPE_CLASS | USB_RECIP_INTERFACE | USB_DIR_OUT
            bRequest: request,
            wValue: value,
            wIndex: index,
            wLength: buffer.len() as u16,
            pData: buffer.as_mut_ptr() as *mut c_void,
            wLenDone: 0,
        };

        let device_request_fn = (**self.device).DeviceRequest.ok_or("DeviceRequest function is null")?;
        let status = device_request_fn(self.device as *mut c_void, &mut req);

        thread::sleep(min_wait);

        if status != 0 {
            return Err(format!("DeviceRequest failed with status: {:#x}", status));
        }

        if req.wLenDone != buffer.len() as u32 {
            return Err("Incomplete transfer".into());
        }

        Ok(status)
    }

    unsafe fn get_feature_report(&mut self, data: &[u8], index: u16, min_wait: Duration, response_length: u16) -> Result<Vec<u8>, String> {
        if self.device.is_null() {
            return Err("Device is null".to_string());
        }

        let mut buffer: Vec<u8> = vec![0; response_length as usize];
        let mut req = IOUSBDevRequest {
            bmRequestType: 0xA1, // USB_TYPE_CLASS | USB_RECIP_INTERFACE | USB_DIR_IN
            bRequest: 0x01,      // HID_REQ_GET_REPORT
            wValue: 0x0300,      // (HID_REPORT_TYPE_FEATURE << 8) | 0x00
            wIndex: index,       //
            wLength: buffer.len() as u16,
            pData: buffer.as_mut_ptr() as *mut c_void,
            wLenDone: 0,
        };

        let device_request_fn = (**self.device).DeviceRequest.ok_or("DeviceRequest function is null")?;
        let status = device_request_fn(self.device as *mut c_void, &mut req);

        thread::sleep(min_wait);

        if status != 0 {
            return Err(format!("DeviceRequest failed with status: {:#x}", status));
        }

        if req.wLenDone != buffer.len() as u32 {
            return Err("Incomplete transfer".into());
        }

        Ok(buffer)
    }

    unsafe fn send_feature_report(&mut self, data: &[u8], index: u16, min_wait: Duration, response_length: u16) -> Result<Vec<u8>, String> {
        if self.device.is_null() {
            return Err("Device is null".to_string());
        }

        if let Err(e) = self.send_control_msg(0x09, 0x300, index, data, min_wait) {
            return Err(format!("Failed to send feature report: {}", e));
        }

        let mut buffer: Vec<u8> = vec![0; response_length as usize];
        let mut req = IOUSBDevRequest {
            bmRequestType: 0x21, // USB_TYPE_CLASS | USB_RECIP_INTERFACE | USB_DIR_OUT
            bRequest: 0x09,      // HID_REQ_SET_REPORT
            wValue: 0x0300,      // (HID_REPORT_TYPE_FEATURE << 8) | 0x00
            wIndex: index,       //
            wLength: buffer.len() as u16,
            pData: buffer.as_mut_ptr() as *mut c_void,
            wLenDone: 0,
        };

        let device_request_fn = (**self.device).DeviceRequest.ok_or("DeviceRequest function is null")?;
        let status = device_request_fn(self.device as *mut c_void, &mut req);

        thread::sleep(min_wait);

        if status != 0 {
            return Err(format!("DeviceRequest failed with status: {:#x}", status));
        }

        if req.wLenDone != buffer.len() as u32 {
            return Err("Incomplete transfer".into());
        }

        Ok(buffer)
    }

    unsafe fn close(&mut self) -> Result<(), String> {
        if self.device.is_null() {
            return Err("Device is null".to_string());
        }

        let result = (**self.device).USBDeviceClose.unwrap()(self.device as *mut c_void);
        if result != kIOReturnSuccess {
            return Err(format!("Failed to close device: {:#x}", result));
        }

        (**self.device).Release.unwrap()(self.device as *mut c_void);
        self.device = ptr::null_mut();

        Ok(())
    }
}
