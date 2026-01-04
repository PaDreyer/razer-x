use bindings::{set_swipe_scroll_direction, CFString, CFBoolean, CFPreferencesSetAppValue, CFPreferencesAppSynchronize, CFPreferencesGetAppBooleanValue, TCFType};
use crate::preferences::PreferencesDriver;

pub struct MacOsPreferencesDriver;

impl MacOsPreferencesDriver {
    pub fn new() -> Self {
        MacOsPreferencesDriver
    }
}

impl PreferencesDriver for MacOsPreferencesDriver {
    fn set_mouse_wheel_inverted(inverted: bool) -> Result<(), String> {
        // CFStrings für Key und App-ID
        let key = CFString::new("com.apple.swipescrolldirection");
        let app_id = CFString::new("NSGlobalDomain");
        let value = if inverted {
            CFBoolean::true_value()
        } else {
            CFBoolean::false_value()
        };

        unsafe {
            CFPreferencesSetAppValue(
                key.as_concrete_TypeRef(),
                value.as_CFTypeRef(),
                app_id.as_concrete_TypeRef(),
            );
            let ok = CFPreferencesAppSynchronize(
                app_id.as_concrete_TypeRef(),
            );
            
            // Synchronisiere die Einstellungen
            set_swipe_scroll_direction(inverted);
            
            if ok > 0 {
                println!("Scrollrichtung (natural) gesetzt auf: {}", inverted);
            } else {
                eprintln!("Fehler beim Schreiben der Scrollrichtung. (oder vielleicht auch schon gesetzt?)");
            }
        }

        /*
        // Wende die Änderung an (aktiviert die Einstellung systemweit)
        let status = Command::new("/System/Library/PrivateFrameworks/SystemAdministrator.framework/activateSettings")
            .arg("-u")
            .status()
            .map_err(|e| format!("Fehler beim Ausführen von activateSettings: {}", e))?;
        */

        /*
        // Neustart von cfprefsd erzwingen
        let status = Command::new("defaults")
            .arg("-w        ")
            .arg("mouse")
            .status();
        match status {
            Ok(status) if status.success() => println!("cfprefsd erfolgreich neu gestartet."),
            Ok(status) => eprintln!("cfprefsd wurde beendet, aber exit code: {}", status),
            Err(err) => eprintln!("Fehler beim Neustart von cfprefsd: {}", err),
        };
         */

        Ok(())
    }

    fn is_mouse_wheel_inverted() -> Result<bool, String> {
        // CFStrings für Key und App-ID
        let key = CFString::new("com.apple.swipescrolldirection");
        let app_id = CFString::new("NSGlobalDomain");

        unsafe {
            // Lese den Wert der Einstellung
            let value = CFPreferencesGetAppBooleanValue(
                key.as_concrete_TypeRef(),
                app_id.as_concrete_TypeRef(),
                std::ptr::null_mut(),
            );

            // Konvertiere den Wert in einen booleschen Wert
            Ok(value != 0)
        }
    }
}
