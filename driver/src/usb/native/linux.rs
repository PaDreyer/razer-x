use super::UsbDriver;

pub struct LinuxUsbDriver;

impl UsbDriver for LinuxUsbDriver {
    fn list_devices(&self) -> Vec<String> {
        unimplemented!("Linux support not yet implemented");
    }

    fn send_feature_report(&self, _data: &[u8]) -> Result<(), String> {
        unimplemented!("Linux support not yet implemented");
    }

    fn read_feature_report(&self, _buf: &mut [u8]) -> Result<usize, String> {
        unimplemented!("Linux support not yet implemented");
    }
}