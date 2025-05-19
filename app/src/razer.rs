
pub const USB_VENDOR_ID_RAZER: u16 = 0x1532;

/* Each USB report has 90 bytes*/
const RAZER_USB_REPORT_LEN: u16 =  0x5A;

const OFF: u16 = 0x00;
const ON : u16 = 0x01;

pub const RAZER_NEW_MOUSE_RECEIVER_WAIT_MIN_US: u16 = 31000;
pub const RAZER_NEW_MOUSE_RECEIVER_WAIT_MAX_US: u16 = 31100;

// LED STORAGE Options
const NOSTORE  : u16 =         0x00;
const VARSTORE : u16 =         0x01;

// LED definitions
const ZERO_LED         : u16 = 0x00;
const SCROLL_WHEEL_LED : u16 = 0x01;
const BATTERY_LED      : u16 = 0x03;
const LOGO_LED         : u16 = 0x04;
const BACKLIGHT_LED    : u16 = 0x05;
const MACRO_LED        : u16 = 0x07;
const GAME_LED         : u16 = 0x08;
const RED_PROFILE_LED  : u16 = 0x0C;
const GREEN_PROFILE_LED: u16 = 0x0D;
const BLUE_PROFILE_LED : u16 = 0x0E;
const RIGHT_SIDE_LED   : u16 = 0x10;
const LEFT_SIDE_LED    : u16 = 0x11;
const ARGB_CH_1_LED    : u16 = 0x1A;
const ARGB_CH_2_LED    : u16 = 0x1B;
const ARGB_CH_3_LED    : u16 = 0x1C;
const ARGB_CH_4_LED    : u16 = 0x1D;
const ARGB_CH_5_LED    : u16 = 0x1E;
const ARGB_CH_6_LED    : u16 = 0x1F;
const CHARGING_LED     : u16 = 0x20;
const FAST_CHARGING_LED: u16 = 0x21;
const FULLY_CHARGED_LED: u16 = 0x22;


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


// Report Responses
const RAZER_CMD_BUSY         : u16 = 0x01;
const RAZER_CMD_SUCCESSFUL   : u16 = 0x02;
const RAZER_CMD_FAILURE      : u16 = 0x03;
const RAZER_CMD_TIMEOUT      : u16 = 0x04;
const RAZER_CMD_NOT_SUPPORTED: u16 = 0x05;

/*
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct TransactionId(u8);

impl PartialEq<Self> for TransactionId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

struct RazerRGB {
    r: u8,
    g: u8,
    b: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct CommandId(u8);

impl PartialEq<Self> for CommandId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

 */

#[derive(Copy, Clone)]
pub struct TransactionId(u8);

impl TransactionId {
    pub fn new(device: u8, id: u8) -> Self {
        assert!(device < 8);
        assert!(id < 32);
        Self((device & 0b0000_0111) | ((id & 0b0001_1111) << 3))
    }

    pub fn raw(&self) -> u8 {
        self.0
    }

    pub fn device(&self) -> u8 {
        self.0 & 0b0000_0111
    }

    pub fn id(&self) -> u8 {
        (self.0 >> 3) & 0b0001_1111
    }
}

#[derive(Copy, Clone)]
pub struct CommandId(u8);

impl CommandId {
    pub fn new(direction: u8, id: u8) -> Self {
        assert!(direction < 2);
        assert!(id < 128);
        Self((direction << 7) | (id & 0x7F))
    }

    pub fn raw(&self) -> u8 {
        self.0
    }

    pub fn direction(&self) -> u8 {
        (self.0 >> 7) & 1
    }

    pub fn id(&self) -> u8 {
        self.0 & 0x7F
    }
}

/* Status:
 * 0x00 New Command
 * 0x01 Command Busy
 * 0x02 Command Successful
 * 0x03 Command Failure
 * 0x04 Command No Response / Command Timeout
 * 0x05 Command Not Support
 *
 * Transaction ID used to group request-response, device useful when multiple devices are on one usb
 * Remaining Packets is the number of remaining packets in the sequence
 * Protocol Type is always 0x00
 * Data Size is the size of payload, cannot be greater than 80. 90 = header (8B) + data + CRC (1B) + Reserved (1B)
 * Command Class is the type of command being issued
 * Command ID is the type of command being send. Direction 0 is Host->Device, Direction 1 is Device->Host. AKA Get LED 0x80, Set LED 0x00
 *
 * */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct RazerReport {
    status: u8,
    transaction_id: TransactionId, /* */
    remaining_packets: u16, /* Big Endian */
    protocol_type: u8, /*0x0*/
    data_size: u8,
    command_class: u8,
    command_id: CommandId,
    pub arguments: [u8; 80],
    pub crc: u8,/*xor'ed bytes of report*/
    reserved: u8, /*0x0*/
}

impl RazerReport {
    /// Gibt die rohen Razer-Bytes ohne Report-ID zurück
    fn raw_bytes(&self) -> [u8; 88] {
        let mut list: [u8; 88] = [0; 88];
        list[0] = self.status;
        list[1] = self.transaction_id.0;

        let be =  self.remaining_packets.to_be_bytes();
        list[2] = be[0];
        list[3] = be[1];
        list[4] = self.protocol_type;
        list[5] = self.data_size;
        list[6] = self.command_class;
        list[7] = self.command_id.0;
        list[8..88].copy_from_slice(&self.arguments);
        list
    }

    /// Serialisiert inklusive Report ID (für HIDAPI)
    pub fn to_hid_bytes(&self) -> Vec<u8> {
        println!("Raw report: {:02X?}", self.raw_bytes());
        let mut buf = Vec::with_capacity(90);
        //buf.push(0x00); // Report ID als erstes Byte
        buf.extend_from_slice(self.raw_bytes().as_ref());
        buf.push(self.crc);
        buf.push(self.reserved);
        buf
    }

    /// Setzt den CRC-Wert korrekt (nach Razer-Logik: XOR über Byte 2–87)
    pub fn finalize(&mut self) {
        let bytes = self.raw_bytes();
        self.crc = bytes[2..88].iter().fold(0u8, |acc, &b| acc ^ b);
    }

    pub fn get_firmware_report() -> Self {
        let mut args = [0u8; 80];
        args[0] = 0x00;
        args[1] = 0x00;

        Self {
            status: 0x0,
            transaction_id: TransactionId(0x1f), //(0x1f),
            remaining_packets: 0x00,
            protocol_type: 0x00,
            data_size: 0x02,
            command_class: 0x00,
            command_id: CommandId(0x81),
            arguments: args,
            crc: 0x00,
            reserved: 0x00,
        }
    }

    pub fn get_poll_rate_report() -> Self {
        Self {
            status: 0x00,
            transaction_id: TransactionId(0x3f),
            remaining_packets: 0x00,
            protocol_type: 0x00,
            data_size: 0x01,
            command_class: 0x00,
            command_id: CommandId(0x85),
            arguments: [0; 80],
            crc: 0x00,
            reserved: 0x00,
        }
    }

    fn get_dpi_report() -> Self {
        Self {
            status: 0x00,
            transaction_id: TransactionId(0x1f),
            remaining_packets: 0x00,
            protocol_type: 0x00,
            data_size: 0x01,
            command_class: 0x00,
            command_id: CommandId(0x84),
            arguments: [0; 80],
            crc: 0x00,
            reserved: 0x00,
        }
    }

    fn get_charging_state_report() -> Self {
        Self {
            status: 0x00,
            transaction_id: TransactionId(0x1f),
            remaining_packets: 0x00,
            protocol_type: 0x00,
            data_size: 0x02,
            command_class: 0x07,
            command_id: CommandId(0x84),
            arguments: [0; 80],
            crc: 0x00,
            reserved: 0x00,
        }
    }
}

impl PartialEq<Self> for RazerReport {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status &&
            self.transaction_id.0 == other.transaction_id.0 &&
            self.remaining_packets == other.remaining_packets &&
            self.protocol_type == other.protocol_type &&
            self.data_size == other.data_size &&
            self.command_class == other.command_class &&
            self.command_id.0 == other.command_id.0 &&
            self.arguments == other.arguments &&
            self.crc == other.crc &&
            self.reserved == other.reserved
    }
}

pub struct RazerARGBReport {
    report_id: u8,
    channel_1: u8,
    channel_2: u8,
    pad: u8,
    last_idx: u8,
    color_data: [u8; 315],
}

pub struct RazerKeyTranslation {
    from: u16,
    to: u16,
    flags: u8,
}


pub fn parse_razer_response(report_id: u8, data: &[u8]) -> Option<RazerReport> {
    if data.len() != 90 {
        eprintln!("Invalid length: expected 90 bytes, got {}", data.len());
        return None;
    }

    // Cast raw bytes to RazerReport
    let mut full_data = [0u8; 90];
    full_data.copy_from_slice(data);

    let report: RazerReport = unsafe {
        std::ptr::read_unaligned(full_data.as_ptr() as *const RazerReport)
    };

    Some(report)
}

pub const RAZER_BASILISK_V3_PRO_ID: u16 = 0x00AB;