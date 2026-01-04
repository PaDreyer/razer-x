use crate::preferences::PreferencesDriver;

pub struct WindowsPreferencesDriver;

impl WindowsPreferencesDriver {
    pub fn new() -> Self {
        WindowsPreferencesDriver
    }
}

impl PreferencesDriver for WindowsPreferencesDriver {
    pub fn set_mouse_wheel_inverted(inverted: bool) -> Result<(), String> {
        unimplemented!("set_mouse_wheel_inverted is not implemented for Windows");
    }

    pub fn is_mouse_wheel_inverted() -> Result<bool, String> {
        unimplemented!("is_mouse_wheel_inverted is not implemented for Windows");
    }
}