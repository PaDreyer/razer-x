use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();

    let (header_path, bindings, maybe_c_code) = match target.as_str() {
        t if t.contains("apple") => {
            let header_path = "native/macos/usb_driver_macos.h";
            let c_file = "native/macos/usb_driver_macos.c";

            // macOS SDK Pfad
            let sdk_path = Command::new("xcrun")
                .args(["--sdk", "macosx", "--show-sdk-path"])
                .output()
                .expect("xcrun fehlgeschlagen");
            let sdk_path = String::from_utf8(sdk_path.stdout).unwrap();
            let sdk_path = sdk_path.trim();

            // Bindgen
            let builder = bindgen::Builder::default()
                .header(header_path)
                .clang_arg(format!("-isysroot{}", sdk_path))
                .clang_arg("-I/System/Library/Frameworks/IOKit.framework/Headers")
                //.clang_arg("-I/System/Library/Frameworks/CoreFoundation.framework/Headers")
                .blocklist_type("REFIID")
                .blocklist_type("CF.*")
                .allowlist_function("IO.*")
                .allowlist_type("IO.*")
                .allowlist_var("kIOCFPlugInInterfaceID")
                .allowlist_var("kIOUSBInterfaceUserClientTypeID")
                .allowlist_function("get_usb_device_uuid")
                .allowlist_function("get_plugin_uuid")
                .allowlist_function("get_usb_device_interface_uuid")
                .allowlist_function("CFRunLoopGetMain")
                .allowlist_function("CFRunLoopWakeUp")
                .allowlist_var("kIOMasterPortDefault")
                .allowlist_var("KERN_SUCCESS")
                .parse_callbacks(Box::new(bindgen::CargoCallbacks));

            // C-Datei kompilieren, falls vorhanden
            cc::Build::new()
                .file(&c_file.to_string())
                .flag(&format!("-isysroot{}", &sdk_path.to_string()))
                .flag("-I/System/Library/Frameworks/IOKit.framework/Headers")
                .flag("-I/System/Library/Frameworks/CoreFoundation.framework/Headers")
                .compile("usb_driver_macos");

            (header_path, builder, Some((c_file.to_string(), sdk_path.to_string())))
        }

        t if t.contains("linux") => {
            let header_path = "native/linux/usb_driver_linux.h";
            let builder = bindgen::Builder::default()
                .header(header_path)
                .parse_callbacks(Box::new(bindgen::CargoCallbacks));
            (header_path, builder, None)
        }

        t if t.contains("windows") => {
            let header_path = "native/windows/usb_driver_windows.h";
            let builder = bindgen::Builder::default()
                .header(header_path)
                .parse_callbacks(Box::new(bindgen::CargoCallbacks));
            (header_path, builder, None)
        }

        _ => panic!("Unsupported platform"),
    };

    // Bindings schreiben
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .generate()
        .expect(&format!("Unable to generate bindings for {}", header_path))
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

    // macOS-spezifisches Linken
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
    }
}
