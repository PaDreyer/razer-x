use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem}
};
pub use tray_icon::TrayIconEvent;
pub use tray_icon::menu::MenuEvent;
use tray_icon::menu::MenuId;

#[derive(Debug, Clone)]
pub enum UserEvent {
    TrayIconEvent(TrayIconEvent),
    MenuEvent(MenuEvent),
}

#[derive(Eq, Hash, PartialEq)]
pub enum MenuElement{
    BatteryStatus,
    PollingRate,
    DpiXY,
    Backlight0,
    Backlight5,
    Backlight25,
    Backlight50,
    Backlight75,
    Backlight100,
    PollingRate1000,
    PollingRate500,
    PollingRate125,
    Quit,
}

pub struct Tray {
    pub(crate) tray_menu: Arc<Mutex<Menu>>,
    pub(crate) tray_icon: Option<Arc<Mutex<TrayIcon>>>,
    pub(crate) icon: Option<Icon>,
    pub(crate) menu_items: HashMap<MenuElement, (MenuId, MenuItem)>,
}

pub type EventHandler<T> = Box<dyn Fn(T) -> Result<(), String> + Send + Sync + 'static>;

impl Tray {
    pub fn new(tray_sender: EventHandler<UserEvent>, menu_sender: EventHandler<UserEvent>) -> Self {
        TrayIconEvent::set_event_handler(Some(move |event| {
            if let Err(e) = tray_sender(UserEvent::TrayIconEvent(event)) {
                eprintln!("Failed to send tray icon event: {}", e);
            }
        }));

        MenuEvent::set_event_handler(Some(move |event| {
            println!("Menu event: {:?}", event);
            if let Err(e) = menu_sender(UserEvent::MenuEvent(event)) {
                eprintln!("Failed to send menu event: {}", e);
            }
        }));

        let tray_menu = Menu::new();
        let mut menu_items: HashMap<MenuElement, (MenuId, MenuItem)> = HashMap::new();

        let battery_status = MenuItem::new("Battery Status: 0%", false, None);
        let polling_rate = MenuItem::new("Polling Rate: 0Hz", false, None);
        let dpi_xy = MenuItem::new("DPI XY: 0x0", false, None);

        let backlight_0 = MenuItem::new("Backlight O%", true, None);
        let backlight_5 = MenuItem::new("Backlight 5%", true, None);
        let backlight_25 = MenuItem::new("Backlight 25%", true, None);
        let backlight_50 = MenuItem::new("Backlight 50%", true, None);
        let backlight_75 = MenuItem::new("Backlight 75%", true, None);
        let backlight_100 = MenuItem::new("Backlight 100%", true, None);

        let polling_rate_1000 = MenuItem::new("Polling Rate: 1000Hz", true, None);
        let polling_rate_500 = MenuItem::new("Polling Rate: 500Hz", true, None);
        let polling_rate_125 = MenuItem::new("Polling Rate: 125Hz", true, None);

        let quit_i = MenuItem::new("Quit", true, None);

        if let Err(e) = tray_menu.append_items(&[
            &battery_status,
            &polling_rate,
            &dpi_xy,
            &PredefinedMenuItem::separator(),
            &backlight_0,
            &backlight_5,
            &backlight_25,
            &backlight_50,
            &backlight_75,
            &backlight_100,
            &PredefinedMenuItem::separator(),
            &polling_rate_1000,
            &polling_rate_500,
            &polling_rate_125,
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::about(
                Some("About Razer-X"),
                Some(AboutMetadata {
                    name: Some("Razer-X".to_string()),
                    copyright: Some("Copyright PLDreyer".to_string()),
                    ..Default::default()
                }),
            ),
            &quit_i,
        ]) {
            println!("Failed to create menu: {}", e);
        }

        menu_items.insert(MenuElement::BatteryStatus, (battery_status.id().clone(), battery_status));
        menu_items.insert(MenuElement::PollingRate, (polling_rate.id().clone(), polling_rate));
        menu_items.insert(MenuElement::DpiXY, (dpi_xy.id().clone(), dpi_xy));

        menu_items.insert(MenuElement::Backlight0, (backlight_0.id().clone(), backlight_0));
        menu_items.insert(MenuElement::Backlight5, (backlight_5.id().clone(), backlight_5));
        menu_items.insert(MenuElement::Backlight25, (backlight_25.id().clone(), backlight_25));
        menu_items.insert(MenuElement::Backlight50, (backlight_50.id().clone(), backlight_50));
        menu_items.insert(MenuElement::Backlight75, (backlight_75.id().clone(), backlight_75));
        menu_items.insert(MenuElement::Backlight100, (backlight_100.id().clone(), backlight_100));

        menu_items.insert(MenuElement::PollingRate1000, (polling_rate_1000.id().clone(), polling_rate_1000));
        menu_items.insert(MenuElement::PollingRate500, (polling_rate_500.id().clone(), polling_rate_500));
        menu_items.insert(MenuElement::PollingRate125, (polling_rate_125.id().clone(), polling_rate_125));

        menu_items.insert(MenuElement::Quit, (quit_i.id().clone(), quit_i));

        let menu_channel = MenuEvent::receiver();
        let tray_channel = TrayIconEvent::receiver();

        Tray {
            tray_menu: Arc::new(Mutex::new(tray_menu)),
            tray_icon: None,
            icon: None,
            menu_items,
        }
    }

    pub fn load_icon(&mut self, path: &std::path::Path) -> Result<(), String> {
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open(path)
                .expect("Failed to open icon path")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
        let icon = tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)
            .expect("Failed to open icon");

        self.icon = Some(icon);

        Ok(())
    }

    pub fn init(&mut self) -> Result<(), String> {
        let mut tray_icon_builder = TrayIconBuilder::new()
                .with_menu(Box::new(self.tray_menu.lock().unwrap().clone()))
                .with_tooltip("tao - awesome windowing lib");

        if let Some(ref icon) = self.icon {
            tray_icon_builder = tray_icon_builder.with_icon(icon.clone());
        }
        
        let tray_icon = tray_icon_builder
            .build()
            .map_err(|e| format!("Failed to create tray icon: {}", e))?;
        
        self.tray_icon = Some(Arc::new(Mutex::new(
            tray_icon
        )));

        // We have to request a redraw here to have the icon actually show up.
        // Tao only exposes a redraw method on the Window so we use core-foundation directly.
        #[cfg(target_os = "macos")]
        unsafe {
            use bindings::{CFRunLoopGetMain, CFRunLoopWakeUp};

            let rl = CFRunLoopGetMain();
            CFRunLoopWakeUp(rl);
        }

        Ok(())
    }

    pub fn is_menu_element<'a>(&self, element: &MenuElement, id: &'a MenuId) -> bool {
        let item = self.menu_items.get(element);
        if let Some(menu_id) = item {
            menu_id.0 == id
        } else {
            false
        }
    }

    pub fn get_menu_item(&self, element: &MenuElement) -> Option<&MenuItem> {
        self.menu_items.get(element).map(|(_, item)| item)
    }

    pub fn set_battery_status(&self, status: u8) -> Result<(), String> {
        assert!(status <= 100, "Battery status must be between 0 and 100");

        let text = format!("Battery Status: {}%", status);
        let menu_item = self.get_menu_item(&MenuElement::BatteryStatus);

        menu_item.unwrap().set_text(text);

        Ok(())
    }

    pub fn set_polling_rate(&self, rate: u16) -> Result<(), String> {
        assert!(rate == 1000 || rate == 500 || rate == 125, "Polling rate must be one of 1000, 500, or 125");

        let text = format!("Polling Rate: {}Hz", rate);
        let menu_item = self.get_menu_item(&MenuElement::PollingRate);

        menu_item.unwrap().set_text(text);

        Ok(())
    }
    
    pub fn set_dpi_xy(&self, dpi_x: u16, dpi_y: u16) -> Result<(), String> {
        let text = format!("DPI XY: {}x{}", dpi_x, dpi_y);
        let menu_item = self.get_menu_item(&MenuElement::DpiXY);

        menu_item.unwrap().set_text(text);

        Ok(())
    }
}