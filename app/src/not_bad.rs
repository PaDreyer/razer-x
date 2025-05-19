mod ior_error;
mod razer;

use ior_error::{init_ior_errors, log_ior_error};

use std::{ffi::CStr, ptr, thread, time::Duration};
use std::os::raw::c_void;
use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use core_foundation_sys::{
    base::kCFAllocatorDefault,
    number::{CFNumberGetValue, kCFNumberSInt32Type},
    string::{CFStringGetCString, kCFStringEncodingUTF8},
    uuid::{CFUUIDBytes, CFUUIDCreateFromUUIDBytes, CFUUIDRef}
};
use core_foundation_sys::uuid::CFUUIDGetUUIDBytes;
use libc::{c_char};
use bindings::{get_usb_device_uuid, get_plugin_uuid, io_registry_entry_t, io_iterator_t, IOIteratorNext,
               IOObjectRelease, IORegistryEntryCreateCFProperty, IOUSBDevRequest, IOUSBDeviceInterface,
               IOCFPlugInInterface, kIOMasterPortDefault, io_service_t, IOCreatePlugInInterfaceForService,
               IOServiceMatching, io_object_t, IOServiceGetMatchingServices};
use crate::ior_error::kIOReturnSuccess;


#[derive(Debug)]
pub struct UsbRegistryEntry {
    pub name: Option<String>,
    pub class: Option<String>,
    pub interface_number: Option<u32>,
    pub entry_id: Option<u32>,
    pub io_service: io_service_t,
    pub children: Vec<UsbRegistryEntry>,
}

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

unsafe fn uuid_from_bytes(bytes: CFUUIDBytes) -> CFUUIDRef {
    CFUUIDCreateFromUUIDBytes(kCFAllocatorDefault, bytes)
}

pub unsafe fn send_feature_report(
    iface: *mut IOUSBDeviceInterface,
    data: &[u8],
    report_index: u16,
    wait_min_us: u64,
    wait_max_us: u64,
) -> Result<(), String> {
    if iface.is_null() {
        return Err("Interface pointer is null".into());
    }

    // Request-Parameter setzen
    let mut buffer = data.to_vec(); // kopieren
    let mut req = IOUSBDevRequest {
        bmRequestType: 0x21, // USB_TYPE_CLASS | USB_RECIP_INTERFACE | USB_DIR_OUT
        bRequest: 0x09,      // HID_REQ_SET_REPORT
        wValue: 0x0300,      // (HID_REPORT_TYPE_FEATURE << 8) | 0x00
        wIndex: report_index,
        wLength: buffer.len() as u16,
        pData: buffer.as_mut_ptr() as *mut c_void,
        wLenDone: 0,
    };

    // Aufruf: DeviceRequest
    let device_request_fn = (*iface).DeviceRequest.ok_or("DeviceRequest function is null")?;
    let status = device_request_fn(iface as *mut c_void, &mut req);

    // Sleep wie bei usleep_range
    let sleep_us = wait_min_us.max(wait_max_us); // oder zufällig innerhalb
    thread::sleep(Duration::from_micros(sleep_us));

    if status != 0 {
        return Err(format!("DeviceRequest failed with status: {:#x}", status));
    }

    if req.wLenDone != buffer.len() as u32 {
        return Err("Incomplete transfer".into());
    }

    Ok(())
}

pub unsafe fn get_feature_report(
    iface: *mut IOUSBDeviceInterface,
    report_index: u16,
    buffer: &mut [u8],
) -> Result<usize, String> {
    if iface.is_null() {
        return Err("Interface pointer is null".into());
    }

    let mut req = IOUSBDevRequest {
        bmRequestType: 0xA1, // USB_TYPE_CLASS | USB_RECIP_INTERFACE | USB_DIR_IN
        bRequest: 0x01,      // HID_REQ_GET_REPORT
        wValue: 0x0300,      // (Feature Report << 8) | Report ID (0x00)
        wIndex: report_index,
        wLength: buffer.len() as u16,
        pData: buffer.as_mut_ptr() as *mut c_void,
        wLenDone: 0,
    };

    let device_request_fn = (*iface).DeviceRequest.ok_or("DeviceRequest function is null")?;
    let status = device_request_fn(iface as *mut c_void, &mut req);

    if status != 0 {
        return Err(format!("DeviceRequest (GET) failed with status: {:#x}", status));
    }

    if req.wLenDone == 0 {
        return Err("No data received".into());
    }

    Ok(req.wLenDone as usize)
}

pub unsafe fn is_razer_device(obj: io_object_t) -> bool {
    let mut vendor: u16 = 0;
    println!("Vendor ID: {:}", vendor);
    println!("Checking if device is Razer...");
    let name = get_string_property(obj, "USB Product Name");
    println!("USB Product Name: {:?}", name);
    let vendor_id = get_int_property(obj, "idVendor");
    println!("Vendor ID: {:?}", vendor_id);

    println!("Vendor ID: {:}", vendor);
    vendor == crate::razer::USB_VENDOR_ID_RAZER
}


pub unsafe fn get_razer_usb_device_interface2() -> *mut IOUSBDeviceInterface {
    let matching_dict = IOServiceMatching(b"IOUSBDevice\0".as_ptr() as *const i8);
    if matching_dict.is_null() {
        return ptr::null_mut();
    }

    let mut iter: io_iterator_t = 0;
    println!("kIoMasterPortDefault: {:?}", kIOMasterPortDefault);
    println!("UsbDevice UUID: {:}", get_usb_device_uuid() as u32);
    println!("Plugin UUID: {:}", get_plugin_uuid() as u32);
    let result = IOServiceGetMatchingServices(kIOMasterPortDefault, matching_dict, &mut iter);
    if result != kIOReturnSuccess {
        return ptr::null_mut();
    }

    println!("Iterating over USB devices...");
    loop {

        let usb_device = IOIteratorNext(iter);

        if usb_device == 0 {
            println!("No more devices found.");
            break;
        }

        let usb_device_name = get_string_property(usb_device, "USB Product Name").unwrap();
        println!("Found USB device: {}", usb_device_name);

        let mut plugin_ptr: *mut IOCFPlugInInterface = ptr::null_mut();
        let mut plugin_ptr_ptr: *mut *mut IOCFPlugInInterface = &mut plugin_ptr;
        let mut score: i32 = 0;

        let kr = IOCreatePlugInInterfaceForService(
            usb_device,
            get_plugin_uuid(),
            get_usb_device_uuid(),
            &mut plugin_ptr_ptr,
            &mut score,
        );

        IOObjectRelease(usb_device);

        if kr != kIOReturnSuccess || (plugin_ptr_ptr as i32) == 0x0 {
            println!("IOCreatePlugInInterfaceForService failed");
            continue;
        }

        let mut device_iface: *mut IOUSBDeviceInterface = ptr::null_mut();

        let mut iface_ptr: *mut IOUSBDeviceInterface = ptr::null_mut();
        let iface_ptr_ptr: *mut *mut IOUSBDeviceInterface = &mut iface_ptr;

        println!("Creating query interface...");
        println!("Plugin pointer: {:?}", **plugin_ptr_ptr);
        let hresult = (**plugin_ptr_ptr).QueryInterface.unwrap()(
            plugin_ptr_ptr as *mut c_void,
            CFUUIDGetUUIDBytes(get_usb_device_uuid()),
            //iface_ptr as *mut *mut c_void,
            iface_ptr_ptr as *mut _ as *mut *mut c_void,
        );

        (**plugin_ptr_ptr).Release.unwrap()(plugin_ptr_ptr as *mut c_void);

        if hresult != 0 || iface_ptr_ptr.is_null() {
            println!("QueryInterface failed");
            continue;
        }

        if !is_razer_device(usb_device) {
            println!("Not a Razer device");
            println!("Device interface pointer: {:?}", **iface_ptr_ptr);
            println!("Release fn: {:?}", (**iface_ptr_ptr).Release.unwrap());
            let iface = *iface_ptr_ptr;
            if !iface.is_null() {
                if let Some(release_fn) = (*iface).Release {
                    let status = release_fn(iface as *mut c_void);
                    println!("✅ Released device interface: {:#x}", status);
                } else {
                    println!("❌ Release function pointer is null");
                }
            }
            println!("Released device interface");
            continue;
        }

        println!("Device is Razer");

        println!("Opening USB device...");
        let open_result = (*device_iface).USBDeviceOpen.unwrap()(device_iface as *mut c_void);
        println!("USBDeviceOpen result: {:#x}", open_result);

        if open_result != kIOReturnSuccess {
            println!("Unable to open USB device: {:08x}", open_result);
            (*device_iface).Release.unwrap()(device_iface as *mut c_void);
            continue;
        }

        IOObjectRelease(iter);
        return device_iface;
    }

    IOObjectRelease(iter);
    ptr::null_mut()
}

fn main() -> anyhow::Result<()> {
    init_ior_errors();
    unsafe {
        let test = get_razer_usb_device_interface2();
    }
    Ok(())
}
