mod usb;
mod preferences;
pub mod settings;
pub mod error;

pub use usb::{UsbDriver, PlatformUsbDriver };
pub use preferences::{PreferencesDriver, PlatformPreferencesDriver};
pub use error::{DriverError};

pub type DriverResult<T> = Result<T, DriverError>;