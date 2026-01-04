#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;


pub trait PreferencesDriver {
    fn set_mouse_wheel_inverted(inverted: bool) -> Result<(), String>;
    fn is_mouse_wheel_inverted() -> Result<bool, String>;
}