use crate::preferences::PreferencesDriver;

pub struct LinuxPreferencesDriver;

impl LinuxPreferencesDriver {
    pub fn new() -> Self {
        LinuxPreferencesDriver
    }
}

impl PreferencesDriver for LinuxPreferencesDriver {
    fn set_mouse_wheel_inverted(inverted: bool) -> Result<(), String> {
        unimplemented!("set_mouse_wheel_inverted is not implemented for Linux");
    }

    fn is_mouse_wheel_inverted() -> Result<bool, String> {
        unimplemented!("is_mouse_wheel_inverted is not implemented for Linux");
    }
}