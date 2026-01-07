// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gui::Gui;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{ptr, thread};
use tauri::async_runtime::JoinHandle;
use tauri::AppHandle;
use ui_lib::Application;

mod mouse;
mod types;

enum AppEvent {
    Close,
    Open,
}

fn main() {
    let app = ui_lib::create_app();
    app.run();
    /*
    let (sender, receiver) = channel::<AppEvent>();

    let app_handle: Arc<Mutex<*const AppHandle>> = Arc::new(Mutex::new(ptr::null_mut::<AppHandle>()));
    let app_handle_clone = Arc::clone(&app_handle);

    let app_thread_handle = thread::spawn(move || {
        let app = ui_lib::create_app();
        *app_handle_clone.lock().unwrap() = app.app.handle();
        app.run();
    });

    thread::sleep(Duration::from_secs(20));
    let test = *app_handle.lock().unwrap();
    unsafe { test.as_ref().unwrap().exit(0); }

    let app_handle_clone = Arc::clone(&app_handle);
    let close_window = move || {
        if let Ok(handle) = app_handle_clone.lock() {
            true
        } else {
            eprintln!("Failed to get app handle");
            false
        }
    };

    let tray = gui::Gui::new(
        get_battery_status,
        set_backlight,
        set_polling_rate,
        get_polling_rate,
        get_dpi_xy,
        close_window,
    );

    tray.run()
     */

    /*
    let thread_handle: Arc<Mutex<Option<thread::JoinHandle<()>>>> = Arc::new(Mutex::new(None));
    let thread_handle_clone = Arc::clone(&thread_handle);

    let (sender, receiver) = channel::<AppEvent>();
    fn create_tray_thread(sender: std::sync::mpsc::Sender<AppEvent>) -> std::thread::JoinHandle<()>{
        std::thread::spawn(move || {
            let sender_ref = Arc::new(Mutex::new(sender));

            let get_battery_status = || {
                unsafe {
                    mouse::get_battery_status()
                }
            };

            let get_polling_rate = || {
                unsafe {
                    mouse::get_polling_rate()
                }
            };

            let set_backlight = |brightness: u8| {
                unsafe {
                    mouse::set_backlight(brightness);
                }
            };

            let set_polling_rate = |rate: u16| {
                unsafe {
                    mouse::set_polling_rate(rate);
                }
            };

            let get_dpi_xy = || {
                unsafe {
                    mouse::get_dpi_xy()
                }
            };

            let sender_clone = Arc::clone(&sender_ref);
            let open_ui = move || {
                sender_clone.lock().unwrap().send(AppEvent::Open).unwrap();
                true
            };

            let sender_clone = Arc::clone(&sender_ref);
            let close_app = move || {
                sender_clone.lock().unwrap().send(AppEvent::Close).unwrap();
                true
            };

            let tray = gui::Gui::new(
                get_battery_status,
                set_backlight,
                set_polling_rate,
                get_polling_rate,
                get_dpi_xy,
                open_ui,
                close_app,
            );

            tray.run();

            return ();
        })
    }

    let tray = create_tray_thread(sender);

    let app = ui_lib::create_app();

    for event in receiver {
        match event {
            AppEvent::Close => {
                println!("Exiting...");
                break;
            },
            AppEvent::Open => {
                println!("Opening UI...");
                break;
            }
        }
    }
     */

    /*
    let ok = tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(move |app| {
            let handle = app.handle().clone();

            let mut app_handle_ref = app_handle_clone.lock().unwrap();
            *app_handle_ref = Some(handle.clone());
            let mut thread_handle_ref = thread_handle.lock().unwrap();

            *thread_handle_ref = Some(std::thread::spawn(move || {
                let webview_window = tauri::WebviewWindowBuilder::new(&handle, "label", tauri::WebviewUrl::App("index.html".into()))
                    .build()
                    .unwrap();
            }));

            Ok(())
        });

    ok.build(tauri::generate_context!()).unwrap().run(|_, _| {});

    let app_handle = app_handle.lock().unwrap();
    if let Some(handle) = &*app_handle {
        handle.exit(0);
    }
    //let app = ui_lib::create_app();
    //app.run();
     */
}
