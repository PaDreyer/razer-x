use crate::DriverResult;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;


pub trait PreferencesDriver {
    fn set_mouse_wheel_inverted(inverted: bool) -> DriverResult<()>;
    fn is_mouse_wheel_inverted() -> DriverResult<bool>;
}