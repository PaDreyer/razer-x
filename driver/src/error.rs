use std::fmt;

#[derive(Debug, Clone)]
pub enum DriverError {
    UsbError(String),
    DeviceNotFound(u16, u16),
    Busy(String),
    IncompleteTransfer,
    InvalidParameter(String),
    IoError(String),
    NotImplemented(String),
    Other(String),
}

impl fmt::Display for DriverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DriverError::UsbError(e) => write!(f, "USB error: {}", e),
            DriverError::DeviceNotFound(v, p) => write!(f, "Device {:04x}:{:04x} not found or busy", v, p),
            DriverError::Busy(e) => write!(f, "Device busy: {}", e),
            DriverError::IncompleteTransfer => write!(f, "Incomplete transfer"),
            DriverError::InvalidParameter(e) => write!(f, "Invalid parameter: {}", e),
            DriverError::IoError(e) => write!(f, "IO error: {}", e),
            DriverError::NotImplemented(e) => write!(f, "Not implemented: {}", e),
            DriverError::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

impl std::error::Error for DriverError {}

impl From<String> for DriverError {
    fn from(s: String) -> Self {
        DriverError::Other(s)
    }
}

impl From<&str> for DriverError {
    fn from(s: &str) -> Self {
        DriverError::Other(s.to_string())
    }
}

impl From<std::io::Error> for DriverError {
    fn from(e: std::io::Error) -> Self {
        DriverError::IoError(e.to_string())
    }
}

impl From<DriverError> for String {
    fn from(e: DriverError) -> Self {
        e.to_string()
    }
}
