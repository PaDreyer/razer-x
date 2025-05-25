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

use crate::consts::RAZER_USB_REPORT_LEN;

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
    pub unsafe fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.len() != RAZER_USB_REPORT_LEN as usize {
            panic!("Invalid length: expected {} bytes, got {}", RAZER_USB_REPORT_LEN, bytes.len());
        }

        // Cast raw bytes to RazerReport
        let mut full_data = [0u8; 90];
        full_data.copy_from_slice(bytes);

        let report: RazerReport = unsafe {
            std::ptr::read_unaligned(full_data.as_ptr() as *const RazerReport)
        };

        report
    }

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

    pub fn to_hid_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(90);

        buf.extend_from_slice(self.raw_bytes().as_ref());
        buf.push(self.crc);
        buf.push(self.reserved);
        
        buf
    }

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
            transaction_id: TransactionId(0x1f),
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

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L1482
    /**
    * Get the polling rate from the device
    *
    * Identifier is in arg[0]
    *
    * 0x01 = 1000Hz
    * 0x02 =  500Hz
    * 0x08 =  125Hz
    */
    pub fn get_poll_rate_report() -> Self {
        Self {
            status: 0x00,
            transaction_id: TransactionId(0x1f),
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

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L1630
    /**
    * Set the polling rate for the device
    *
    * Identifier is in arg[0]
    *
    * 1000 = 0x01
    * 500  = 0x02 (50Hz)
    * 125  = 0x08
    */
    pub fn set_poll_rate_report(polling_rate: u16) -> Self {
        let mut arguments = [0u8; 80];
        arguments[0] = match polling_rate {
            1000 => 0x01,
            500 => 0x02,
            125 => 0x08,
            _ => panic!("Invalid polling rate. Must be 1000, 500 or 125"),
        };

        Self {
            status: 0x00,
            transaction_id: TransactionId(0x1f),
            remaining_packets: 0x00,
            protocol_type: 0x00,
            data_size: 0x01,
            command_class: 0x00,
            command_id: CommandId(0x05),
            arguments,
            crc: 0x00,
            reserved: 0x00,
        }
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L2055
    /**
    * let dpi_x: u16 = ((response.arguments[1] as u16) << 8) | (response.arguments[2] as u16 & 0xFF);
    * let dpi_y: u16 = ((response.arguments[3] as u16) << 8) | (response.arguments[4] as u16 & 0xFF);
    */
    pub fn get_dpi_xy_report() -> Self {
        Self {
            status: 0x00,
            transaction_id: TransactionId(0x1f),
            remaining_packets: 0x00,
            protocol_type: 0x00,
            data_size: 0x07,
            command_class: 0x04,
            command_id: CommandId(0x85),
            arguments: [0; 80],
            crc: 0x00,
            reserved: 0x00,
        }
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L1873
    pub fn set_dpi_xy_report(dpi_x: u16, dpi_y: u16) -> Self {
        // verify that dpi_x and dpi_y are in the range of 100-35000
        assert!(dpi_x >= 100 && dpi_x <= 35000, "dpi_x doesn't fit between 100 and 35000");
        assert!(dpi_y >= 100 && dpi_y <= 35000, "dpi_y doesn't fit between 100 and 35000");

        let mut arguments = [0u8; 80];
        arguments[0] = 0x01; // VARSTORE
        arguments[1] = ((dpi_x >> 8) & 0x00FF) as u8;
        arguments[2] = (dpi_x & 0x00FF) as u8;
        arguments[3] = ((dpi_y >> 8) & 0x00FF) as u8;
        arguments[4] = (dpi_y & 0x00FF) as u8;
        arguments[5] = 0x00;
        arguments[6] = 0x00;

        Self {
            status: 0x00,
            transaction_id: TransactionId(0x1f),
            remaining_packets: 0x00,
            protocol_type: 0x00,
            data_size: 0x07,
            command_class: 0x04,
            command_id: CommandId(0x05),
            arguments,
            crc: 0x00,
            reserved: 0x00,
        }
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L1331
    pub fn get_charging_state_report() -> Self {
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
    
    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L1257
    /**
    * Gets battery level
    *
    * 0->255 is in arg[1]
    * Returns an integer which needs to be scaled from 0-255 -> 0-100
    */
    pub fn get_battery_level_report() -> Self {
        Self {
            status: 0x00,
            transaction_id: TransactionId(0x1f),
            remaining_packets: 0x00,
            protocol_type: 0x00,
            data_size: 0x02,
            command_class: 0x07,
            command_id: CommandId(0x80),
            arguments: [0; 80],
            crc: 0x00,
            reserved: 0x00,
        }
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L1812
    /**
    * Response: Brightness is at arg[0] for dock and arg[1] for led_brightness
    */
    pub fn get_matrix_brightness_report() -> Self {
        let mut arguments = [0u8; 80];
        arguments[0] = 0x01;
        arguments[1] = 0x00;

        Self {
            status: 0x00,
            transaction_id: TransactionId(0x1f),
            remaining_packets: 0x00,
            protocol_type: 0x00,
            data_size: 0x03,
            command_class: 0x0F,
            command_id: CommandId(0x84),
            arguments,
            crc: 0x00,
            reserved: 0x00,
        }
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L1756
    pub fn set_matrix_brightness_report(brightness: u8) -> Self {
        let mut arguments = [0u8; 80];
        
        arguments[0] = 0x01; // VARSTORE
        arguments[1] = 0x00; // ZERO_LED
        arguments[2] = brightness;;
        
        Self {
            status: 0x00,
            transaction_id: TransactionId(0x1f),
            remaining_packets: 0x00,
            protocol_type: 0x00,
            data_size: 0x03,
            command_class: 0x0F,
            command_id: CommandId(0x04),
            arguments,
            crc: 0x00,
            reserved: 0x00,
        }
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L3212
    pub fn get_led_brightness_report(led_id: u8) -> Self {
        let mut arguments = [0u8; 80];
        arguments[0] = 0x01;
        arguments[1] = led_id;

        Self {
            status: 0x00,
            transaction_id: TransactionId(0x1f),
            remaining_packets: 0x00,
            protocol_type: 0x00,
            data_size: 0x03,
            command_class: 0x0F,
            command_id: CommandId(0x84),
            arguments: [0; 80],
            crc: 0x00,
            reserved: 0x00,
        }
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L4143
    fn set_led_brightness_report() -> Self {
        unimplemented!()
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L2311
    pub fn get_scroll_smart_reel_report() -> Self {
        let mut arguments = [0u8; 80];
        arguments[0] = 0x01;

        Self {
            status: 0x00,
            transaction_id: TransactionId(0x1f),
            remaining_packets: 0x00,
            protocol_type: 0x00,
            data_size: 0x02,
            command_class: 0x02,
            command_id: CommandId(0x97),
            arguments,
            crc: 0x00,
            reserved: 0x00,
        }
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L2288
    fn set_scroll_smart_reel_report() -> Self {
        unimplemented!()
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L2269
    fn get_scroll_acceleration_report() -> Self {
        unimplemented!()
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L2246
    fn set_scroll_acceleration_report() -> Self {
        unimplemented!()
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L2510
    fn get_dpi_stages_report() -> Self {
        unimplemented!()
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L2400
    fn set_dpi_stages_report() -> Self {
        unimplemented!()
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L2610
    /**
    * idle_time = (response.arguments[0] << 8) | (response.arguments[1] & 0xFF);
    */
    pub fn get_idle_timeout_report() -> Self {
        Self {
            status: 0x00,
            transaction_id: TransactionId(0x1f),
            remaining_packets: 0x00,
            protocol_type: 0x00,
            data_size: 0x02,
            command_class: 0x07,
            command_id: CommandId(0x83),
            arguments: [0; 80],
            crc: 0x00,
            reserved: 0x00,
        }
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L2667
    fn set_idle_timeout_report() -> Self {
        unimplemented!()
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L2721
    pub fn get_charge_low_threshold_report() -> Self {
        Self {
            status: 0x00,
            transaction_id: TransactionId(0x1f),
            remaining_packets: 0x00,
            protocol_type: 0x00,
            data_size: 0x01,
            command_class: 0x07,
            command_id: CommandId(0x81),
            arguments: [0; 80],
            crc: 0x00,
            reserved: 0x00,
        }
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L2766
    fn set_charge_low_threshold_report() -> Self {
        unimplemented!()
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L3040
    pub fn get_device_mode_report() -> Self {
        Self {
            status: 0x00,
            transaction_id: TransactionId(0x1f),
            remaining_packets: 0x00,
            protocol_type: 0x00,
            data_size: 0x02,
            command_class: 0x00,
            command_id: CommandId(0x84),
            arguments: [0; 80],
            crc: 0x00,
            reserved: 0x00,
        }
    }

    // https://github.com/openrazer/openrazer/blob/master/driver/razermouse_driver.c#L2942
    fn set_device_mode_report() -> Self {
        unimplemented!()
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

pub struct RazerKeyTranslation {
    from: u16,
    to: u16,
    flags: u8,
}
