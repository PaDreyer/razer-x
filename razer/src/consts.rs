const OFF: u16 = 0x00;
const ON : u16 = 0x01;

// LED STORAGE Options
pub const NOSTORE  : u8 =         0x00;
pub const VARSTORE : u8 =         0x01;

pub const RAZER_USB_VENDOR_ID: u16 = 0x1532;
pub const RAZER_BASILISK_V3_PRO_ID: u16 = 0x00AB;
pub const RAZER_USB_REPORT_LEN: u8 =  0x5A;

pub const RAZER_CMD_BUSY         : u8 = 0x01;
pub const RAZER_CMD_SUCCESSFUL   : u8 = 0x02;
pub const RAZER_CMD_FAILURE      : u8 = 0x03;
pub const RAZER_CMD_TIMEOUT      : u8 = 0x04;
pub const RAZER_CMD_NOT_SUPPORTED: u8 = 0x05;

pub const RAZER_NEW_MOUSE_RECEIVER_WAIT_MIN_US: u16 = 31000;
pub const RAZER_NEW_MOUSE_RECEIVER_WAIT_MAX_US: u16 = 31100;

// LED definitions
pub const ZERO_LED         : u8 = 0x00;
pub const SCROLL_WHEEL_LED : u8 = 0x01;
pub const BATTERY_LED      : u8 = 0x03;
pub const LOGO_LED         : u8 = 0x04;
pub const BACKLIGHT_LED    : u8 = 0x05;
pub const MACRO_LED        : u8 = 0x07;
pub const GAME_LED         : u8 = 0x08;
pub const RED_PROFILE_LED  : u8 = 0x0C;
pub const GREEN_PROFILE_LED: u8 = 0x0D;
pub const BLUE_PROFILE_LED : u8 = 0x0E;
pub const RIGHT_SIDE_LED   : u8 = 0x10;
pub const LEFT_SIDE_LED    : u8 = 0x11;
pub const ARGB_CH_1_LED    : u8 = 0x1A;
pub const ARGB_CH_2_LED    : u8 = 0x1B;
pub const ARGB_CH_3_LED    : u8 = 0x1C;
pub const ARGB_CH_4_LED    : u8 = 0x1D;
pub const ARGB_CH_5_LED    : u8 = 0x1E;
pub const ARGB_CH_6_LED    : u8 = 0x1F;
pub const CHARGING_LED     : u8 = 0x20;
pub const FAST_CHARGING_LED: u8 = 0x21;
pub const FULLY_CHARGED_LED: u8 = 0x22;

enum RazerClassicEffectId {
    ClassicEffectStatic = 0x00,
    ClassicEffectBlinking = 0x01,
    ClassicEffectBreathing = 0x02, // also called pulsating
    ClassicEffectSpectrum = 0x04,
}

enum RazerMatrixEffectId {
    MatrixEffectOff = 0x00,
    MatrixEffectWave = 0x01,
    MatrixEffectReactive = 0x02, // afterglow
    MatrixEffectBreathing = 0x03,
    MatrixEffectSpectrum = 0x04,
    MatrixEffectCustomFrame = 0x05,
    MatrixEffectStatic = 0x06,
    MatrixEffectStarlight = 0x19,
}
