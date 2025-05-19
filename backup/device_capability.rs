pub enum DeviceCapability {
    MouseCapability(MouseCapability),
    KeyboardCapability(KeyboardCapability),
}

pub enum MouseCapability {
    DPI,
    PollingRate,
    RGB,
    Brightness,
}

pub enum KeyboardCapability {
    RGB,
}