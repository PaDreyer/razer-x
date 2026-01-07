// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::mpsc::channel;
mod mouse;
mod types;

enum AppEvent {
    Close,
    Open,
}

fn main() {
    let app = ui_lib::create_app();
    app.run();
}
