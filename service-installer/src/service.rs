// service.rs

#[cfg(target_os = "linux")]
pub use linux::SystemdServiceInstaller as PlatformServiceInstaller;
#[cfg(target_os = "macos")]
pub use macos::LaunchdServiceInstaller as PlatformServiceInstaller;
#[cfg(target_os = "windows")]
pub use windows::WindowsServiceInstaller as PlatformServiceInstaller;

pub trait ServiceInstaller {
    fn install(&self) -> Result<(), String>;
    fn uninstall(&self) -> Result<(), String>;
    fn is_installed(&self) -> bool;
}

#[cfg(target_os = "linux")]
pub mod linux {
    use super::ServiceInstaller;
    use std::fs;
    use std::path::PathBuf;

    pub struct SystemdServiceInstaller {
        service_name: String,
    }

    impl SystemdServiceInstaller {
        pub fn new(service_name: &str) -> Self {
            Self { service_name: service_name.to_string() }
        }

        fn user_unit_path(&self) -> PathBuf {
            dirs::home_dir()
                .unwrap()
                .join(".config/systemd/user")
                .join(format!("{}.service", self.service_name))
        }
    }

    impl ServiceInstaller for SystemdServiceInstaller {
        fn install(&self) -> Result<(), String> {
            let path = self.user_unit_path();
            fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;

            let content = format!(
                "[Unit]\nDescription=Rust app: {}\n\n[Service]\nExecStart={}\nRestart=on-failure\n\n[Install]\nWantedBy=default.target\n",
                self.service_name,
                std::env::current_exe().unwrap().display()
            );
            fs::write(&path, content).map_err(|e| e.to_string())?;

            std::process::Command::new("systemctl")
                .args(["--user", "daemon-reload"])
                .status().map_err(|e| e.to_string())?;
            std::process::Command::new("systemctl")
                .args(["--user", "enable", &self.service_name])
                .status().map_err(|e| e.to_string())?;
            Ok(())
        }

        fn uninstall(&self) -> Result<(), String> {
            let path = self.user_unit_path();
            std::process::Command::new("systemctl")
                .args(["--user", "disable", &self.service_name])
                .status().map_err(|e| e.to_string())?;
            fs::remove_file(&path).ok();
            Ok(())
        }

        fn is_installed(&self) -> bool {
            self.user_unit_path().exists()
        }
    }
}

#[cfg(target_os = "macos")]
pub mod macos {
    use super::ServiceInstaller;
    use std::fs;
    use std::path::PathBuf;

    pub struct LaunchdServiceInstaller {
        label: String,
    }

    impl LaunchdServiceInstaller {
        pub fn new(label: &str) -> Self {
            Self { label: label.to_string() }
        }

        fn plist_path(&self) -> PathBuf {
            dirs::home_dir()
                .unwrap()
                .join("Library/LaunchAgents")
                .join(format!("{}.plist", self.label))
        }
    }

    impl ServiceInstaller for LaunchdServiceInstaller {
        fn install(&self) -> Result<(), String> {
            let path = self.plist_path();
            fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;

            let content = format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\">\n<dict>\n    <key>Label</key><string>{}</string>\n    <key>ProgramArguments</key>\n    <array><string>{}</string></array>\n    <key>RunAtLoad</key><true/>\n</dict>\n</plist>",
                self.label,
                std::env::current_exe().unwrap().display()
            );

            fs::write(&path, content).map_err(|e| e.to_string())?;
            std::process::Command::new("launchctl")
                .args(["load", "-w", &path.display().to_string()])
                .status().map_err(|e| e.to_string())?;
            Ok(())
        }

        fn uninstall(&self) -> Result<(), String> {
            let path = self.plist_path();
            std::process::Command::new("launchctl")
                .args(["unload", "-w", &path.display().to_string()])
                .status().map_err(|e| e.to_string())?;
            fs::remove_file(&path).ok();
            Ok(())
        }

        fn is_installed(&self) -> bool {
            self.plist_path().exists()
        }
    }
}

#[cfg(target_os = "windows")]
pub mod windows {
    use super::ServiceInstaller;
    use windows_service::service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType};
    use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};
    use windows_service::service_dispatcher;

    pub struct WindowsServiceInstaller {
        name: String,
    }

    impl WindowsServiceInstaller {
        pub fn new(name: &str) -> Self {
            Self { name: name.to_string() }
        }
    }

    impl ServiceInstaller for WindowsServiceInstaller {
        fn install(&self) -> Result<(), String> {
            let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CREATE_SERVICE)
                .map_err(|e| format!("ServiceManager error: {e:?}"))?;

            let service_binary_path = std::env::current_exe().map_err(|e| e.to_string())?;

            let info = ServiceInfo {
                name: self.name.clone().into(),
                display_name: self.name.clone().into(),
                service_type: ServiceType::OWN_PROCESS,
                start_type: ServiceStartType::AutoStart,
                error_control: ServiceErrorControl::Normal,
                executable_path: service_binary_path,
                launch_arguments: vec![],
                dependencies: vec![],
                account_name: None, // LocalSystem
                account_password: None,
            };

            manager.create_service(&info, ServiceAccess::START)
                .map_err(|e| format!("CreateService error: {e:?}"))?;

            Ok(())
        }

        fn uninstall(&self) -> Result<(), String> {
            let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)
                .map_err(|e| format!("ServiceManager error: {e:?}"))?;

            let service = manager.open_service(self.name.clone(), ServiceAccess::DELETE)
                .map_err(|e| format!("OpenService error: {e:?}"))?;

            service.delete().map_err(|e| format!("DeleteService error: {e:?}"))?;
            Ok(())
        }

        fn is_installed(&self) -> bool {
            let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT);
            if let Ok(mgr) = manager {
                mgr.open_service(self.name.clone(), ServiceAccess::QUERY_STATUS).is_ok()
            } else {
                false
            }
        }
    }
}
