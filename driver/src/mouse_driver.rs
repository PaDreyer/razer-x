use crate::driver::{ Driver };

pub trait MouseDriver: Driver {
    const NAME: &'static str;

    fn new() -> Self;
    
    fn set_dpi(&self, dpi: u32) -> Result<(), String>;
    
    fn set_polling_rate(&self, rate: u32) -> Result<(), String>;
    
    fn set_rgb(&self, r: u8, g: u8, b: u8) -> Result<(), String>;
    
    fn set_brightness(&self, brightness: u8) -> Result<(), String>;
    
    fn get_dpi(&self) -> Result<u32, String>;
    
    fn get_polling_rate(&self) -> Result<u32, String>;
    
    fn get_rgb(&self) -> Result<(u8, u8, u8), String>;
    
    
    
    
}
