use std::os::raw::c_void;
use std::os::unix::io::AsRawFd;
use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
    sync::{Mutex, OnceLock},
    thread,
    time::Duration,
};

use super::{Device, UsbDriver};
use crate::{DriverError, DriverResult};

// ioctl macros and constants for hidraw
// These are calculated for 91 bytes: 1 byte Report ID + 90 bytes Razer Report
const HIDIOCSFEATURE: u64 = 0xC05B4806; // _IOWR('H', 0x06, 91)
const HIDIOCGFEATURE: u64 = 0xC05B4807; // _IOWR('H', 0x07, 91)

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

pub struct LinuxUsbDriver {
    file: fs::File,
    _path: PathBuf,
    _vendor_id: u16,
    _product_id: u16,
}

unsafe impl Send for LinuxUsbDriver {}
unsafe impl Sync for LinuxUsbDriver {}

impl LinuxUsbDriver {
    fn find_hidraw_device(vendor_id: u16, product_id: u16) -> Result<PathBuf, String> {
        let entries = fs::read_dir("/sys/class/hidraw").map_err(|e| e.to_string())?;

        let mut interface_0 = None;

        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let name = entry.file_name().into_string().unwrap();
            let uevent_path = entry.path().join("device/uevent");

            if let Ok(uevent) = fs::read_to_string(&uevent_path) {
                let vid_str = format!("{:04X}", vendor_id);
                let pid_str = format!("{:04X}", product_id);

                if uevent.to_uppercase().contains(&vid_str)
                    && uevent.to_uppercase().contains(&pid_str)
                {
                    let path = PathBuf::from("/dev").join(name);

                    if uevent.contains("input0") || uevent.contains(":1.0") {
                        interface_0 = Some(path);
                        break; // We only care about Interface 0
                    }
                }
            }
        }

        if let Some(path) = interface_0 {
            return Ok(path);
        }

        eprintln!(
            "DRIVER ERROR: Hidraw device not found for VID:{:04X} PID:{:04X}",
            vendor_id, product_id
        );
        Err(format!(
            "Hidraw device not found for VID:{:04X} PID:{:04X}",
            vendor_id, product_id
        ))
    }
}

impl UsbDriver for LinuxUsbDriver {
    unsafe fn new(vendor_id: u16, product_id: u16) -> DriverResult<Self> {
        let path = match Self::find_hidraw_device(vendor_id, product_id) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("DRIVER ERROR: find_hidraw_device failed: {}", e);
                return Err(DriverError::DeviceNotFound(vendor_id, product_id));
            }
        };

        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .map_err(|e| {
                eprintln!("DRIVER ERROR: Failed to open {}: {}", path.display(), e);
                DriverError::UsbError(format!("Failed to open {}: {}", path.display(), e))
            })?;

        Ok(Self {
            file,
            _path: path,
            _vendor_id: vendor_id,
            _product_id: product_id,
        })
    }

    unsafe fn list_devices() -> Vec<Device> {
        let mut devices = vec![];
        let entries = match fs::read_dir("/sys/class/hidraw") {
            Ok(e) => e,
            Err(e) => {
                eprintln!("DRIVER ERROR: Failed to read /sys/class/hidraw: {}", e);
                return devices;
            }
        };

        for entry in entries {
            if let Ok(entry) = entry {
                let uevent_path = entry.path().join("device/uevent");
                if let Ok(uevent) = fs::read_to_string(&uevent_path) {
                    if let Some(hid_id_line) = uevent.lines().find(|l| l.starts_with("HID_ID=")) {
                        let parts: Vec<&str> = hid_id_line[7..].split(':').collect();
                        if parts.len() >= 3 {
                            let vendor_id = u32::from_str_radix(parts[1], 16).unwrap_or(0);
                            let product_id = u32::from_str_radix(parts[2], 16).unwrap_or(0);

                            if vendor_id == 0x1532 {
                                let name_line = uevent.lines().find(|l| l.starts_with("HID_NAME="));
                                let name =
                                    name_line.map(|l| l[9..].to_string()).unwrap_or_else(|| {
                                        format!("{:04X}:{:04X}", vendor_id, product_id)
                                    });

                                if !devices
                                    .iter()
                                    .any(|d| d.vendor_id == vendor_id && d.product_id == product_id)
                                {
                                    devices.push(Device {
                                        name,
                                        vendor_id,
                                        product_id,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        devices
    }

    unsafe fn send_control_msg(
        &mut self,
        _request: u8,
        _value: u16,
        _index: u16,
        data: &[u8],
        min_wait: Duration,
    ) -> DriverResult<()> {
        let fd = self.file.as_raw_fd();

        // HIDIOCSFEATURE(91) - including report ID
        let mut buf = vec![0u8; data.len() + 1];
        buf[0] = 0x00; // Report ID
        buf[1..].copy_from_slice(data);

        let res = libc::ioctl(fd, HIDIOCSFEATURE, buf.as_ptr());

        if res < 0 {
            let errno = *libc::__errno_location();
            eprintln!(
                "DRIVER ERROR: HIDIOCSFEATURE ioctl failed with res: {}, errno: {}",
                res, errno
            );
            // Fallback: simple write()
            match self.file.write_all(&buf) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("DRIVER ERROR: hidraw write fallback failed: {}", e);
                    return Err(DriverError::UsbError(format!("Hidraw write failed: {}", e)));
                }
            }
        }

        thread::sleep(min_wait);
        Ok(())
    }

    unsafe fn get_feature_report(
        &mut self,
        data: &[u8],
        _index: u16,
        min_wait: Duration,
        response_length: u16,
    ) -> DriverResult<Vec<u8>> {
        // First send the command
        self.send_control_msg(0x09, 0x300, 0, data, min_wait)?;

        let fd = self.file.as_raw_fd();

        // HIDIOCGFEATURE(91)
        let mut buf = vec![0u8; 91];
        buf[0] = 0x00; // Expected report ID 0

        let res = libc::ioctl(fd, HIDIOCGFEATURE, buf.as_mut_ptr());

        if res < 0 {
            let errno = *libc::__errno_location();
            eprintln!(
                "DRIVER ERROR: HIDIOCGFEATURE ioctl failed with res: {}, errno: {}",
                res, errno
            );
            // Fallback: simple read()
            let mut read_buf = vec![0u8; response_length as usize + 1];
            match self.file.read(&mut read_buf) {
                Ok(n) => {
                    thread::sleep(min_wait);
                    return Ok(read_buf[1..n].to_vec());
                }
                Err(e) => {
                    eprintln!("DRIVER ERROR: hidraw read fallback failed: {}", e);
                    return Err(DriverError::UsbError(format!("Hidraw read failed: {}", e)));
                }
            }
        }

        thread::sleep(min_wait);
        Ok(buf[1..].to_vec())
    }

    unsafe fn close(&mut self) -> DriverResult<()> {
        // File closed on drop
        Ok(())
    }

    fn on_device_connected<F>(vendor_id: u16, product_id: u16, callback: F) -> DriverResult<()>
    where
        F: FnMut(&Device) + Send + 'static,
    {
        let mut registry = get_registry().lock().unwrap();
        registry
            .connected_callbacks
            .push((vendor_id, product_id, Box::new(callback)));
        Ok(())
    }

    fn on_device_disconnected<F>(vendor_id: u16, product_id: u16, callback: F) -> DriverResult<()>
    where
        F: FnMut(&Device) + Send + 'static,
    {
        let mut registry = get_registry().lock().unwrap();
        registry
            .disconnected_callbacks
            .push((vendor_id, product_id, Box::new(callback)));
        Ok(())
    }

    fn on_state_changed<F>(&mut self, _callback: F) -> DriverResult<()>
    where
        F: FnMut(&Device, &mut c_void) + Send + 'static,
    {
        Err(DriverError::NotImplemented(
            "State changes not implemented for hidraw".into(),
        ))
    }
}

impl Drop for LinuxUsbDriver {
    fn drop(&mut self) {
        unsafe {
            let _ = self.close();
        }
    }
}
