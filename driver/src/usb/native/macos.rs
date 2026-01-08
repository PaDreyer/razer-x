use std::ffi::CStr;
use std::os::raw::{c_void, c_char};
use std::{ptr, thread};
use std::time::Duration;
use super::{Device, UsbDriver};
use crate::{DriverResult, DriverError};
use bindings::{
    kIOReturnSuccess, 
    IOServiceMatching,
    IOServiceGetMatchingServices,
    IOIteratorNext,
    IOObjectRelease,
    IOCFPlugInInterface,
    IOCreatePlugInInterfaceForService,
    IOServiceAddMatchingNotification,
    
    CFUUIDGetUUIDBytes,
    kIOMasterPortDefault,
    io_iterator_t,
    get_plugin_uuid,
    get_usb_device_uuid,
    get_usb_device_interface_uuid,
    io_registry_entry_t,
    IORegistryEntryCreateCFProperty,
    kCFStringEncodingUTF8,
    CFStringGetCString,
    CFString,
    TCFType,
    IOUSBDeviceInterface,
    IOUSBDevRequest,
    CFNumberGetValue,
    kCFNumberSInt32Type,
    IONotificationPortCreate,
    IONotificationPortGetRunLoopSource,
    IONotificationPortDestroy,
    CFRunLoopAddSource,
    CFRunLoopGetCurrent,
    CFRunLoopRun,
    kIOFirstMatchNotification,
    kIOTerminatedNotification,
    CFRetain,
};
use log::{debug, error, warn};
use std::sync::{Mutex, OnceLock};

type HotplugCallback = Box<dyn FnMut(&Device) + Send + 'static>;

struct HotplugRegistry {
    connected_callbacks: Vec<(u16, u16, HotplugCallback)>,
    disconnected_callbacks: Vec<(u16, u16, HotplugCallback)>,
}

static REGISTRY: OnceLock<Mutex<HotplugRegistry>> = OnceLock::new();

fn get_registry() -> &'static Mutex<HotplugRegistry> {
    REGISTRY.get_or_init(|| {
        Mutex::new(HotplugRegistry {
            connected_callbacks: Vec::new(),
            disconnected_callbacks: Vec::new(),
        })
    })
}

unsafe extern "C" fn device_notification_callback(
    _refcon: *mut c_void,
    iterator: io_iterator_t,
) {
    loop {
        let usb_device = IOIteratorNext(iterator);
        if usb_device == 0 {
            break;
        }

        let vendor_id = get_int_property(usb_device, "idVendor")
            .or_else(|| get_int_property(usb_device, "VendorID"))
            .unwrap_or(0) as u16;
        let product_id = get_int_property(usb_device, "idProduct")
            .or_else(|| get_int_property(usb_device, "ProductID"))
            .unwrap_or(0) as u16;

        // Filter: We only want the primary "Mouse" HID interface (UsagePage 1, Usage 2)
        // This avoids multiple events for the same composite device and also
        // correctly handles wireless mouse connection/disconnection logical events.
        let usage_page = get_int_property(usb_device, "PrimaryUsagePage").unwrap_or(0);
        let usage = get_int_property(usb_device, "PrimaryUsage").unwrap_or(0);
        
        // If it's an HID device (usage_page > 0), check if it's the mouse interface.
        // We also allow it if it has no usage info (likely a raw USB device notification, if we still had those).
        if usage_page != 0 && (usage_page != 1 || usage != 2) {
            IOObjectRelease(usb_device);
            continue;
        }

        let name = get_string_property(usb_device, "USB Product Name")
            .or_else(|| get_string_property(usb_device, "Product"))
            .unwrap_or_else(|| "Unknown".to_string());

        let device = Device {
            name,
            vendor_id: vendor_id as u32,
            product_id: product_id as u32,
        };

        // This is a bit tricky because we don't know if it's connected or disconnected here
        // without passing more info in refcon.
        // Actually, the iterator is specific to the notification type.
        
        let mut registry = get_registry().lock().unwrap();
        
        // We need to know which list to trigger. 
        // Let's use the refcon to pass a boolean: true for connected, false for disconnected.
        let is_connected = !(_refcon as usize == 0);

        if is_connected {
            for (v, p, cb) in registry.connected_callbacks.iter_mut() {
                if (*v == 0 || *v == vendor_id) && (*p == 0 || *p == product_id) {
                    cb(&device);
                }
            }
        } else {
            for (v, p, cb) in registry.disconnected_callbacks.iter_mut() {
                if (*v == 0 || *v == vendor_id) && (*p == 0 || *p == product_id) {
                    cb(&device);
                }
            }
        }

        IOObjectRelease(usb_device);
    }
}

fn ensure_notification_thread() {
    static THREAD_STARTED: OnceLock<()> = OnceLock::new();
    THREAD_STARTED.get_or_init(|| {
        thread::spawn(|| unsafe {
            let notify_port = IONotificationPortCreate(kIOMasterPortDefault);
            if notify_port.is_null() {
                error!("Failed to create IONotificationPort");
                return;
            }

            let run_loop_source = IONotificationPortGetRunLoopSource(notify_port);
            let run_loop = CFRunLoopGetCurrent();
            CFRunLoopAddSource(run_loop, run_loop_source, bindings::kCFRunLoopDefaultMode);

            // We listen for IOHIDDevice notifications instead of IOUSBDevice.
            // This is because IOHIDDevice events fire not only when the physical dongle is (un)plugged,
            // but also when a wireless mouse logically connects/disconnects from its receiver.
            let matching_dict = IOServiceMatching(b"IOHIDDevice\0".as_ptr() as *const i8);
            if matching_dict.is_null() {
                error!("Failed to create matching dictionary for hotplug");
                return;
            }

            let matching_dict_disconnect = CFRetain(matching_dict as _);

            let mut connected_iter: io_iterator_t = 0;
            let mut disconnected_iter: io_iterator_t = 0;

            // Connect notification
            IOServiceAddMatchingNotification(
                notify_port,
                kIOFirstMatchNotification.as_ptr() as *const i8,
                matching_dict,
                Some(device_notification_callback),
                1 as *mut c_void, // connected = true
                &mut connected_iter,
            );
            device_notification_callback(1 as *mut c_void, connected_iter);

            // Disconnect notification
            IOServiceAddMatchingNotification(
                notify_port,
                kIOTerminatedNotification.as_ptr() as *const i8,
                matching_dict_disconnect as _,
                Some(device_notification_callback),
                0 as *mut c_void, // connected = false
                &mut disconnected_iter,
            );
            device_notification_callback(0 as *mut c_void, disconnected_iter);

            CFRunLoopRun();
            
            // Cleanup if it ever stops
            IONotificationPortDestroy(notify_port);
            IOObjectRelease(connected_iter);
            IOObjectRelease(disconnected_iter);
        });
    });
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


pub struct MacOsUsbDriver {
    _vendor_id: u16,
    _product_id: u16,
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
    unsafe fn new(vendor_id: u16, product_id: u16) -> DriverResult<Self> {
        let matching_dict = IOServiceMatching(b"IOUSBDevice\0".as_ptr() as *const i8);
        if matching_dict.is_null() {
            return Err(DriverError::UsbError("IOServiceMatching failed".into()));
        }

        let mut iter: io_iterator_t = 0;
        let result = IOServiceGetMatchingServices(kIOMasterPortDefault, matching_dict, &mut iter);
        if result != kIOReturnSuccess {
            return Err(DriverError::UsbError("IOServiceGetMatchingServices failed".into()));
        }

        let device;
        loop {
            let usb_device = IOIteratorNext(iter);

            if usb_device == 0 {
                device = None;
                break;
            }

            let device_vendor_id = get_int_property(usb_device, "idVendor").unwrap_or(0);
            let device_product_id = get_int_property(usb_device, "idProduct").unwrap_or(0);
            
            if device_vendor_id != vendor_id as u32 || device_product_id != product_id as u32 {
                IOObjectRelease(usb_device);
                continue;
            }

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
                warn!("IOCreatePlugInInterfaceForService failed");
                continue;
            }

            let mut iface_ptr: *mut IOUSBDeviceInterface = ptr::null_mut();
            let mut iface_ptr_ptr: *mut *mut IOUSBDeviceInterface = &mut iface_ptr;
            let iface_ptr_ptr_ptr: *mut *mut *mut IOUSBDeviceInterface = &mut iface_ptr_ptr;

            let hresult = (**plugin_interface_ptr).QueryInterface.unwrap()(
                plugin_interface_ptr as *mut c_void,
                CFUUIDGetUUIDBytes(get_usb_device_interface_uuid()),
                iface_ptr_ptr_ptr as *mut *mut c_void,
            );

            (**plugin_interface_ptr).Release.unwrap()(plugin_interface_ptr as *mut c_void);

            if hresult != 0 || iface_ptr_ptr.is_null() {
                warn!("QueryInterface failed");
                continue;
            }

            if (**iface_ptr_ptr).USBDeviceOpen.is_none() {
                warn!("USBDeviceOpen function pointer is null");
                (**iface_ptr_ptr).Release.unwrap()(iface_ptr_ptr as *mut c_void);
                continue;
            }

            // Retry mechanism for opening the device
            let mut open_result = kIOReturnSuccess;
            for attempt in 0..5 {
                open_result = (**iface_ptr_ptr).USBDeviceOpen.unwrap()(iface_ptr_ptr as *mut c_void);
                if open_result == kIOReturnSuccess {
                    break;
                }
                
                // e00002c5 is kIOReturnExclusiveAccess
                if open_result == 0xe00002c5u32 as i32 {
                     debug!("Device busy (exclusive access), retrying in 200ms... (attempt {})", attempt + 1);
                     thread::sleep(Duration::from_millis(200));
                     continue;
                }

                break;
            }

            if open_result != kIOReturnSuccess {
                error!("Unable to open USB device: {:08x}", open_result);
                (**iface_ptr_ptr).Release.unwrap()(iface_ptr_ptr as *mut c_void);
                continue;
            }

            device = Some(iface_ptr_ptr);
            break;
        }
        
        IOObjectRelease(iter);

        match device {
            Some(dev) => Ok(Self {
                _vendor_id: vendor_id,
                _product_id: product_id,
                device: dev,
            }),
            None => Err(DriverError::DeviceNotFound(vendor_id, product_id)),
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
                break;
            }

            let usb_device_name = get_string_property(usb_device, "USB Product Name").unwrap();
            let vendor_id = get_int_property(usb_device, "idVendor").unwrap();
            let product_id = get_int_property(usb_device, "idProduct").unwrap();

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

    unsafe fn send_control_msg(&mut self, request: u8, value: u16, index: u16, data: &[u8], min_wait: Duration) -> DriverResult<()> {
        if self.device.is_null() {
            return Err(DriverError::UsbError("Device is null".to_string()));
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
            return Err(DriverError::UsbError(format!("DeviceRequest failed with status: {:#x}", status)));
        }

        if req.wLenDone != buffer.len() as u32 {
            return Err(DriverError::IncompleteTransfer);
        }

        Ok(())
    }

    unsafe fn get_feature_report(&mut self, data: &[u8], index: u16, min_wait: Duration, response_length: u16) -> DriverResult<Vec<u8>> {
        if self.device.is_null() {
            return Err(DriverError::UsbError("Device is null".to_string()));
        }

        if let Err(e) = self.send_control_msg(0x09, 0x300, index, data, min_wait) {
            return Err(DriverError::UsbError(format!("Failed to send feature report: {}", e)));
        }
        
        thread::sleep(min_wait);

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
            return Err(DriverError::UsbError(format!("DeviceRequest failed with status: {:#x}", status)));
        }

        if req.wLenDone != buffer.len() as u32 {
            return Err(DriverError::IncompleteTransfer);
        }

        Ok(buffer)
    }
    
    unsafe fn close(&mut self) -> DriverResult<()> {
        if self.device.is_null() {
            return Err(DriverError::UsbError("Device is null".to_string()));
        }

        let result = (**self.device).USBDeviceClose.unwrap()(self.device as *mut c_void);
        if result != kIOReturnSuccess {
            return Err(DriverError::UsbError(format!("Failed to close device: {:#x}", result)));
        }

        (**self.device).Release.unwrap()(self.device as *mut c_void);
        self.device = ptr::null_mut();

        Ok(())
    }

    fn on_device_connected<F>(vendor_id: u16, product_id: u16, callback: F) -> DriverResult<()>
    where
        F: FnMut(&Device) + Send + 'static,
    {
        ensure_notification_thread();
        let mut registry = get_registry().lock().unwrap();
        registry.connected_callbacks.push((vendor_id, product_id, Box::new(callback)));
        Ok(())
    }

    fn on_device_disconnected<F>(vendor_id: u16, product_id: u16, callback: F) -> DriverResult<()>
    where
        F: FnMut(&Device) + Send + 'static,
    {
        ensure_notification_thread();
        let mut registry = get_registry().lock().unwrap();
        registry.disconnected_callbacks.push((vendor_id, product_id, Box::new(callback)));
        Ok(())
    }

    fn on_state_changed<F>(&mut self, _callback: F) -> DriverResult<()>
    where
        F: FnMut(&Device, &mut c_void) + Send + 'static,
    {
        Err(DriverError::NotImplemented("State change notifications are not implemented for macOS".into()))
    }
}
