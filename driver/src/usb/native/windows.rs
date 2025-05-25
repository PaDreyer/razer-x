use super::UsbDriver;

pub struct WindowsUsbDriver;

impl UsbDriver for WindowsUsbDriver {
    fn list_devices(&self) -> Vec<String> {
        unimplemented!("Windows support not yet implemented");
    }

    fn read_feature_report(&self, _buf: &mut [u8]) -> Result<usize, String> {
        unimplemented!("Windows support not yet implemented");
    }
}
