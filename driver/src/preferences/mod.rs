mod native;

#[cfg(target_os = "macos")]
mod preferences_impl {
    pub use super::native::macos::MacOsPreferencesDriver as PlatformPreferencesDriver;
}

#[cfg(target_os = "linux")]
mod preferences_impl {
    pub use super::native::linux::LinuxPreferencesDriver as PlatformPreferencesDriver;
}

#[cfg(target_os = "windows")]
mod preferences_impl {
    pub use super::native::windows::WindowsPreferencesDriver as PlatformPreferencesDriver;
}

pub use preferences_impl::PlatformPreferencesDriver;
pub use native::PreferencesDriver;
