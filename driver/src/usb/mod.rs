mod native;

#[cfg(target_os = "macos")]
mod usb_impl {
    pub use super::native::macos::MacOsUsbDriver as PlatformUsbDriver;
}

#[cfg(target_os = "linux")]
mod usb_impl {
    pub use super::native::linux::LinuxUsbDriver as PlatformUsbDriver;
}

pub use usb_impl::PlatformUsbDriver;
pub use native::UsbDriver;