use crate::{RazerReport, RAZER_USB_REPORT_LEN};

mod keyboard;
mod mouse;

pub trait RazerDevice {
    unsafe fn from_bytes(bytes: &[u8]) -> Self {
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
        //buf.push(0x00); // Report ID als erstes Byte
        buf.extend_from_slice(self.raw_bytes().as_ref());
        buf.push(self.crc);
        buf.push(self.reserved);
        buf
    }

    pub fn finalize(&mut self) {
        let bytes = self.raw_bytes();
        self.crc = bytes[2..88].iter().fold(0u8, |acc, &b| acc ^ b);
    }
}