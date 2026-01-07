use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();

    let (header_path, builder) = match target.as_str() {
        t if t.contains("apple") => configure_macos(),
        t if t.contains("linux") => configure_linux(),
        t if t.contains("windows") => configure_windows(),
        _ => panic!("Unsupported platform: {}", target),
    };

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    builder
        .generate()
        .expect(&format!("Unable to generate bindings for {}", header_path))
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

fn configure_macos() -> (&'static str, bindgen::Builder) {
    let header_path = "native/macos/usb_driver_macos.h";
    let c_file = "native/macos/usb_driver_macos.c";

    let sdk_path = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("xcrun failed to get SDK path");
    let sdk_path = String::from_utf8(sdk_path.stdout).unwrap();
    let sdk_path = sdk_path.trim();

    let builder = bindgen::Builder::default()
        .header(header_path)
        .clang_arg(format!("-isysroot{}", sdk_path))
        .blocklist_type("REFIID")
        .blocklist_type("CF.*")
        .allowlist_function("IO.*")
        .allowlist_type("IO.*")
        .allowlist_var("kIOCFPlugInInterfaceID")
        .allowlist_var("kIOUSBInterfaceUserClientTypeID")
        .allowlist_function("get_usb_device_uuid")
        .allowlist_function("get_plugin_uuid")
        .allowlist_function("get_usb_device_interface_uuid")
        .allowlist_function("set_swipe_scroll_direction")
        .allowlist_function("CFRunLoopGetCurrent")
        .allowlist_function("CFRunLoopGetMain")
        .allowlist_function("CFRunLoopAddSource")
        .allowlist_function("CFRunLoopRun")
        .allowlist_function("CFRunLoopWakeUp")
        .allowlist_function("CFRunLoopStop")
        .allowlist_function("IONotificationPort.*")
        .allowlist_function("IOServiceAddMatchingNotification")
        .allowlist_var("kIOMasterPortDefault")
        .allowlist_var("kIOMainPortDefault")
        .allowlist_var("kIOFirstMatchNotification")
        .allowlist_var("kIOTerminatedNotification")
        .allowlist_var("KERN_SUCCESS")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    cc::Build::new()
        .file(c_file)
        .compile("usb_driver_macos");

    println!("cargo:rustc-link-lib=framework=IOKit");

    (header_path, builder)
}

fn configure_linux() -> (&'static str, bindgen::Builder) {
    println!("cargo:rustc-link-lib=usb-1.0");
    println!("cargo:rustc-link-search=native=/usr/lib");

    let header_path = "native/linux/usb_driver_linux.h";
    let builder = bindgen::Builder::default()
        .header(header_path)
        .clang_arg("-I/usr/include")
        .clang_arg("-I/usr/include/libusb-1.0")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));
    (header_path, builder)
}

fn configure_windows() -> (&'static str, bindgen::Builder) {
    let header_path = "native/windows/usb_driver_windows.h";
    let builder = bindgen::Builder::default()
        .header(header_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));
    (header_path, builder)
}
