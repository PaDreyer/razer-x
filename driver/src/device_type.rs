pub(crate) enum DeviceType {
    Mouse,
    Keyboard,
    Touchpad,
    Touchscreen,
    Joystick,
    Gamepad,
    Trackball,
    Trackpoint,
    GraphicsTablet,
    RemoteControl,
    Other(String),
}