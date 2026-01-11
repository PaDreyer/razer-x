use crate::preferences::PreferencesDriver;
use crate::{DriverResult, DriverError};

pub struct LinuxPreferencesDriver;

impl LinuxPreferencesDriver {
    pub fn new() -> Self {
        LinuxPreferencesDriver
    }
}

impl PreferencesDriver for LinuxPreferencesDriver {
    fn set_mouse_wheel_inverted(_inverted: bool) -> DriverResult<()> {
        Err(DriverError::NotImplemented("set_mouse_wheel_inverted is not implemented for Linux".into()))
    }

    fn is_mouse_wheel_inverted() -> DriverResult<bool> {
        Err(DriverError::NotImplemented("is_mouse_wheel_inverted is not implemented for Linux".into()))
    }
}