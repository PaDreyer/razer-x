pub mod tray;
pub mod run_loop;

use std::path::Path;
use tao::event::Event;
use crate::run_loop::RunLoop;
use crate::tray::{Tray, EventHandler, UserEvent, TrayIconEvent, MenuElement, MenuEvent};

enum SystemEvent {}

enum UnifiedEvent {
    User(tray::UserEvent),
    System(SystemEvent),
}


pub struct Gui {
    run_loop: RunLoop<UnifiedEvent>,
    tray: Tray,
    get_battery_status: Box<dyn FnMut() -> u8>,
    set_backlight: Box<dyn FnMut(u8)>,
    set_polling_rate: Box<dyn FnMut(u16)>,
    get_polling_rate: Box<dyn FnMut() -> u16>,
    get_dpi_xy: Box<dyn FnMut() -> (u16, u16)>,
}

impl Gui {
    pub fn new(
        get_battery_status: impl FnMut() -> u8 + 'static,
        set_backlight: impl FnMut(u8) + 'static,
        set_polling_rate: impl FnMut(u16) + 'static,
        get_polling_rate: impl FnMut() -> u16 + 'static,
        get_dpi_xy: impl FnMut() -> (u16, u16) + 'static,
        open_ui: impl FnMut() -> bool + 'static,
        close_app: impl FnMut() -> bool + 'static,
    ) -> Self {
        let run_loop = RunLoop::new();

        // Proxy events from tray menu to the run loop
        let menu_proxy = run_loop.create_proxy();
        let menu_sender: EventHandler<UserEvent> = Box::new(move |event| {
            if let Err(e) = menu_proxy.send_event(UnifiedEvent::User(event)) {
                eprintln!("Failed to send menu event: {}", e);
            }

            Ok(())
        });

        // Proxy events from tray icon to the run loop
        let tray_sender = run_loop.create_proxy();
        let tray_sender: EventHandler<UserEvent> = Box::new(move |event| {
            if let Err(e) = tray_sender.send_event(UnifiedEvent::User(event)) {
                eprintln!("Failed to send tray icon event: {}", e);
            }

            Ok(())
        });

        // Create the tray with the menu and tray event handlers
        let tray = Tray::new(menu_sender, tray_sender);

        Gui {
            run_loop,
            tray,
            get_battery_status: Box::new(get_battery_status),
            set_backlight: Box::new(set_backlight),
            set_polling_rate: Box::new(set_polling_rate),
            get_polling_rate: Box::new(get_polling_rate),
            get_dpi_xy: Box::new(get_dpi_xy),
        }
    }
    
    pub fn run(mut self) -> ! {
        // Start runloop and handle events
        self.run_loop.run(move |event, exit| {
            match event {
                Event::NewEvents(tao::event::StartCause::Init) => {
                    #[cfg(target_os = "macos")]
                    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/img_1.png");
                    
                    #[cfg(target_os = "linux")]
                    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/img.png");
                    
                    if let Err(e) = self.tray.load_icon(Path::new(path)) {
                        eprintln!("Failed to load icon: {}", e);
                    }

                    self.tray.init().unwrap()
                }
                Event::UserEvent(UnifiedEvent::User(UserEvent::TrayIconEvent(TrayIconEvent::Enter{ id, position, rect}))) => {
                    let battery_status = (self.get_battery_status)();
                    self.tray.set_battery_status(battery_status).unwrap();

                    let polling_rate = (self.get_polling_rate)();
                    self.tray.set_polling_rate(polling_rate).unwrap();

                    let (dpi_x, dpi_y) = (self.get_dpi_xy)();
                    self.tray.set_dpi_xy(dpi_x, dpi_y).unwrap();
                }
                Event::UserEvent(UnifiedEvent::User(UserEvent::TrayIconEvent(event))) => {
                    println!("Tray icon: {:?}", event);
                }
                Event::UserEvent(UnifiedEvent::User(UserEvent::MenuEvent(event))) => {
                    let menu_id = event.id.clone();

                    if self.tray.is_menu_element(&MenuElement::Quit, &menu_id) {
                        exit();
                        return;
                    }

                    let mut backlight: Option<u8> = None;
                    let mut polling_rate: Option<u16> = None;

                    if self.tray.is_menu_element(&MenuElement::Backlight0, &menu_id) {
                        backlight = Some(0);
                    }

                    if self.tray.is_menu_element(&MenuElement::Backlight5, &menu_id) {
                        backlight = Some(5);
                    }

                    if self.tray.is_menu_element(&MenuElement::Backlight25, &menu_id) {
                        backlight = Some(25);
                    }

                    if self.tray.is_menu_element(&MenuElement::Backlight50, &menu_id) {
                        backlight = Some(50);
                    }

                    if self.tray.is_menu_element(&MenuElement::Backlight75, &menu_id) {
                        backlight = Some(75);
                    }

                    if self.tray.is_menu_element(&MenuElement::Backlight100, &menu_id) {
                        backlight = Some(100);
                    }

                    if self.tray.is_menu_element(&MenuElement::PollingRate1000, &menu_id) {
                        polling_rate = Some(1000);
                    }

                    if self.tray.is_menu_element(&MenuElement::PollingRate500, &menu_id) {
                        polling_rate = Some(500);
                    }

                    if self.tray.is_menu_element(&MenuElement::PollingRate125, &menu_id) {
                        polling_rate = Some(125);
                    }

                    if let Some(polling_rate) = polling_rate {
                        (self.set_polling_rate)(polling_rate);
                    }

                    if let Some(brightness) = backlight {
                        (self.set_backlight)(brightness);
                    }
                }
                _ => {}
            }
        });
    }
}