use std::os::raw::c_void;
use std::time::Duration;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;


#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub vendor_id: u32,
    pub product_id: u32,
}

pub trait UsbDriver {
    unsafe fn new(vendor_id: u16, product_id: u16) -> Self;

    unsafe fn list_devices() -> Vec<Device>;
    unsafe fn send_control_msg(&mut self, request: u8, value: u16, index: u16, data: &[u8], min_wait: Duration) -> Result<(), String>;

    unsafe fn get_feature_report(&mut self, data: &[u8], index: u16, min_wait: Duration, response_length: u16) -> Result<Vec<u8>, String>;

    unsafe fn close(&mut self) -> Result<(), String>;
    
    fn on_device_connected<F>(vendor_id: u16, product_id: u16, callback: F) -> Result<(), String>
    where
        F: FnMut(&Device) + Send + 'static;
    
    fn on_device_disconnected<F>(vendor_id: u16, product_id: u16, callback: F) -> Result<(), String> 
    where
        F: FnMut(&Device) + Send + 'static;
    
    fn on_state_changed<F>(&mut self, callback: F) -> Result<(), String>
    where
        F: FnMut(&Device, &mut c_void) + Send + 'static;
}