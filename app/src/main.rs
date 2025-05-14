use std::{ffi::CStr, ptr, thread, time::Duration};
use std::os::raw::c_void;
use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use core_foundation_sys::{
    base::kCFAllocatorDefault,
    number::{CFNumberGetValue, kCFNumberSInt32Type},
    string::{CFStringGetCString, kCFStringEncodingUTF8},
    uuid::{CFUUIDBytes, CFUUIDCreateFromUUIDBytes, CFUUIDRef}
};
use libc::c_char;
use bindings::{
    io_registry_entry_t, io_iterator_t, IOIteratorNext, IOObjectRelease, IORegistryEntryCreateCFProperty, 
    IORegistryEntryGetChildIterator, IOUSBDevRequest, IOUSBDeviceInterface, IORegistryGetRootEntry, IORegistryEntryFromPath,
    IOCFPlugInInterface, kIOMasterPortDefault, KERN_SUCCESS, io_service_t, IOCreatePlugInInterfaceForService};


const USB_VENDOR_ID_RAZER: u16 = 0x1532;

/* Each USB report has 90 bytes*/
const RAZER_USB_REPORT_LEN: u16 =  0x5A;

const OFF: u16 = 0x00;
const ON : u16 = 0x01;

const RAZER_NEW_MOUSE_RECEIVER_WAIT_MIN_US: u16 = 31000;
const RAZER_NEW_MOUSE_RECEIVER_WAIT_MAX_US: u16 = 31100;

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
struct RazerReport {
    status: u8,
    transaction_id: TransactionId, /* */
    remaining_packets: u16, /* Big Endian */
    protocol_type: u8, /*0x0*/
    data_size: u8,
    command_class: u8,
    command_id: CommandId,
    arguments: [u8; 80],
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
        let mut buf = Vec::with_capacity(91);
        buf.push(0x00); // Report ID als erstes Byte
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

    fn get_firmware_report() -> Self {
        let mut args = [0u8; 80];
        args[0] = 0x00;
        args[1] = 0x00;

        Self {
            status: 0x0,
            transaction_id: TransactionId(0x5A), //(0x1f),
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

    fn get_poll_rate_report() -> Self {
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

struct RazerARGBReport {
    report_id: u8,
    channel_1: u8,
    channel_2: u8,
    pad: u8,
    last_idx: u8,
    color_data: [u8; 315],
}

struct RazerKeyTranslation {
    from: u16,
    to: u16,
    flags: u8,
}


fn parse_razer_response(report_id: u8, data: &[u8]) -> Option<RazerReport> {
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

const RAZER_BASILISK_V3_PRO_ID: u16 = 0x00AB;

#[derive(Debug)]
pub struct UsbRegistryEntry {
    pub name: Option<String>,
    pub class: Option<String>,
    pub interface_number: Option<u32>,
    pub entry_id: Option<u32>,
    pub io_service: io_service_t,
    pub children: Vec<UsbRegistryEntry>,
}

unsafe fn get_string_property(entry: io_registry_entry_t, key: &str) -> Option<String> {
    let cf_key = CFString::new(key);
    let raw = IORegistryEntryCreateCFProperty(
        entry,
        cf_key.as_concrete_TypeRef(),
        ptr::null_mut(),
        0,
    );
    if raw.is_null() {
        return None;
    }

    let mut buf = [0 as c_char; 256];
    let success = CFStringGetCString(
        raw as _,
        buf.as_mut_ptr(),
        buf.len() as isize,
        kCFStringEncodingUTF8,
    );
    if success == 0 {
        return None;
    }

    Some(CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned())
}

unsafe fn get_int_property(entry: io_registry_entry_t, key: &str) -> Option<u32> {
    let cf_key = CFString::new(key);
    let raw = IORegistryEntryCreateCFProperty(
        entry,
        cf_key.as_concrete_TypeRef(),
        ptr::null_mut(),
        0,
    );
    if raw.is_null() {
        return None;
    }

    let mut val: i32 = 0;
    let success = CFNumberGetValue(raw as _, kCFNumberSInt32Type, &mut val as *mut _ as *mut _);
    if success != false {
        Some(val as u32)
    } else {
        None
    }
}

/// Gibt einen Eintrag zurück – aber nur wenn er sinnvolle Daten enthält oder sinnvolle Kinder hat
unsafe fn build_registry_tree_filtered(entry: io_registry_entry_t) -> Option<UsbRegistryEntry> {
    let name = get_string_property(entry, "USB Product Name");
    let class = get_string_property(entry, "IOClass");
    let interface_number = get_int_property(entry, "bInterfaceNumber");

    let mut children: Vec<UsbRegistryEntry> = Vec::new();
    let mut child_iter: io_iterator_t = 0;

    let result = IORegistryEntryGetChildIterator(
        entry,
        b"IOService\0".as_ptr() as *const i8,
        &mut child_iter,
    );

    if result == KERN_SUCCESS as i32 {
        loop {
            let child = IOIteratorNext(child_iter);
            if child == 0 {
                break;
            }

            if let Some(child_entry) = build_registry_tree_filtered(child) {
                children.push(child_entry);
            }

            IOObjectRelease(child);
        }
        IOObjectRelease(child_iter);
    }

    // Behalte nur sinnvolle Nodes
    let is_useful = name.is_some() || class.is_some() || interface_number.is_some() || !children.is_empty();
    if is_useful {
        Some(UsbRegistryEntry {
            name,
            class,
            interface_number,
            entry_id: Some(entry),
            io_service: entry,
            children,
        })
    } else {
        None
    }
}

unsafe fn find_device_tree_by_name(target: &str) -> Option<UsbRegistryEntry> {
    let root = IORegistryGetRootEntry(kIOMasterPortDefault);
    if root == 0 {
        panic!("❌ Failed to get IORegistry root");
    }

    let entry = IORegistryEntryFromPath(kIOMasterPortDefault, b"IOUSB:/\0".as_ptr() as *const i8);
    if entry == 0 {
        println!("❌ IORegistryEntryFromPath failed");
        return None;
    }

    let mut stack = vec![entry];
    while let Some(current) = stack.pop() {
        let name = get_string_property(current, "USB Product Name");
        if let Some(ref actual_name) = name {
            if actual_name == target {
                let tree = build_registry_tree_filtered(current);
                IOObjectRelease(current);
                return tree;
            }
        }

        let mut children: io_iterator_t = 0;
        let result = IORegistryEntryGetChildIterator(
            current,
            b"IOService\0".as_ptr() as *const i8,
            &mut children,
        );

        if result == KERN_SUCCESS as i32 {
            loop {
                let child = IOIteratorNext(children);
                if child == 0 {
                    break;
                }
                stack.push(child);
            }
            IOObjectRelease(children);
        }

        IOObjectRelease(current);
    }

    None
}

fn find_interface_by_number(
    entry: &UsbRegistryEntry,
    iface_num: u32,
) -> Option<&UsbRegistryEntry> {
    if entry.interface_number == Some(iface_num) {
        return Some(entry);
    }

    for child in &entry.children {
        if let Some(found) = find_interface_by_number(child, iface_num) {
            return Some(found);
        }
    }

    None
}

fn print_device_tree(entry: &UsbRegistryEntry, depth: usize) {
    let indent = "  ".repeat(depth);
    let mut line = String::new();

    if let Some(name) = &entry.name {
        line.push_str(&format!("🖱️ {}", name));
    }

    if let Some(iface) = entry.interface_number {
        line.push_str(&format!("  [iface: {}]", iface));
    }

    if let Some(class) = &entry.class {
        line.push_str(&format!("  (class: {})", class));
    }

    if !line.is_empty() {
        println!("{}{}", indent, line);
    }

    for child in &entry.children {
        print_device_tree(child, depth + 1);
    }
}

unsafe fn get_usb_device_interface(entry: io_service_t) -> Result<*mut IOUSBDeviceInterface, String> {
    let mut plugin_ptr: *mut IOCFPlugInInterface = std::ptr::null_mut();
    let mut plugin_ptr_ptr: *mut *mut IOCFPlugInInterface = &mut plugin_ptr;
    let mut plugin_ptr_ptr_ptr: *mut *mut *mut IOCFPlugInInterface = &mut plugin_ptr_ptr;
    let mut score: i32 = 0;

    // CFUUID für kIOCFPlugInInterfaceID
    let plugin_uuid = uuid_from_bytes(CFUUIDBytes {
        byte0: 0xC9, byte1: 0xA0, byte2: 0x5E, byte3: 0x92,
        byte4: 0xCA, byte5: 0x16, byte6: 0x11, byte7: 0xD0,
        byte8: 0xBD, byte9: 0x52, byte10: 0x00, byte11: 0xC0,
        byte12: 0x4F, byte13: 0xD9, byte14: 0x44, byte15: 0x05,
    });

    // CFUUID für IOUSBDeviceInterface
    let interface_uuid = uuid_from_bytes(CFUUIDBytes {
        byte0: 0x5C, byte1: 0x81, byte2: 0x87, byte3: 0xD0,
        byte4: 0x9E, byte5: 0xF3, byte6: 0x11, byte7: 0xD4,
        byte8: 0x8B, byte9: 0x45, byte10: 0x00, byte11: 0x0A,
        byte12: 0x27, byte13: 0x05, byte14: 0x28, byte15: 0x61,
    });

    // Plugin-Schnittstelle erstellen
    let result = IOCreatePlugInInterfaceForService(
        entry,
        plugin_uuid,
        plugin_uuid, // ⚠️ Das ist korrekt: beide Male der Plug-in-UUID
        plugin_ptr_ptr_ptr,
        &mut score,
    );

    if result != 0 || plugin_ptr.is_null() {
        println!("Result: {}", result);
        println!("Plugin pointer: {:?}", plugin_ptr);
        return Err("IOCreatePlugInInterfaceForService failed".into());
    }

    // Interface abrufen
    let mut device_iface: *mut IOUSBDeviceInterface = std::ptr::null_mut();
    let iface_ptr: *mut *mut c_void = &mut device_iface as *mut _ as *mut *mut c_void;

    let hr = (*plugin_ptr).QueryInterface.unwrap()(
        plugin_ptr as *mut c_void,
        interface_uuid,
        iface_ptr,
    );

    if hr != 0 || device_iface.is_null() {
        return Err("QueryInterface for IOUSBDeviceInterface failed".into());
    }

    Ok(device_iface)
}

fn find_all_interfaces<'a>(entry: &'a UsbRegistryEntry) -> Vec<&'a UsbRegistryEntry> {
    let mut result = Vec::new();

    if entry.interface_number.is_some() {
        result.push(entry);
    }

    for child in &entry.children {
        result.extend(find_all_interfaces(child));
    }

    result
}

unsafe fn uuid_from_bytes(bytes: CFUUIDBytes) -> CFUUIDRef {
    CFUUIDCreateFromUUIDBytes(kCFAllocatorDefault, bytes)
}

pub unsafe fn send_feature_report(
    iface: *mut IOUSBDeviceInterface,
    data: &[u8],
    report_index: u16,
    wait_min_us: u64,
    wait_max_us: u64,
) -> Result<(), String> {
    if iface.is_null() {
        return Err("Interface pointer is null".into());
    }

    // Request-Parameter setzen
    let mut buffer = data.to_vec(); // kopieren
    let mut req = IOUSBDevRequest {
        bmRequestType: 0x21, // USB_TYPE_CLASS | USB_RECIP_INTERFACE | USB_DIR_OUT
        bRequest: 0x09,      // HID_REQ_SET_REPORT
        wValue: 0x0300,      // (HID_REPORT_TYPE_FEATURE << 8) | 0x00
        wIndex: report_index,
        wLength: buffer.len() as u16,
        pData: buffer.as_mut_ptr() as *mut c_void,
        wLenDone: 0,
    };

    // Aufruf: DeviceRequest
    let device_request_fn = (*iface).DeviceRequest.ok_or("DeviceRequest function is null")?;
    let status = device_request_fn(iface as *mut c_void, &mut req);

    // Sleep wie bei usleep_range
    let sleep_us = wait_min_us.max(wait_max_us); // oder zufällig innerhalb
    thread::sleep(Duration::from_micros(sleep_us));

    if status != 0 {
        return Err(format!("DeviceRequest failed with status: {:#x}", status));
    }

    if req.wLenDone != buffer.len() as u32 {
        return Err("Incomplete transfer".into());
    }

    Ok(())
}

pub unsafe fn get_feature_report(
    iface: *mut IOUSBDeviceInterface,
    report_index: u16,
    buffer: &mut [u8],
) -> Result<usize, String> {
    if iface.is_null() {
        return Err("Interface pointer is null".into());
    }

    let mut req = IOUSBDevRequest {
        bmRequestType: 0xA1, // USB_TYPE_CLASS | USB_RECIP_INTERFACE | USB_DIR_IN
        bRequest: 0x01,      // HID_REQ_GET_REPORT
        wValue: 0x0300,      // (Feature Report << 8) | Report ID (0x00)
        wIndex: report_index,
        wLength: buffer.len() as u16,
        pData: buffer.as_mut_ptr() as *mut c_void,
        wLenDone: 0,
    };

    let device_request_fn = (*iface).DeviceRequest.ok_or("DeviceRequest function is null")?;
    let status = device_request_fn(iface as *mut c_void, &mut req);

    if status != 0 {
        return Err(format!("DeviceRequest (GET) failed with status: {:#x}", status));
    }

    if req.wLenDone == 0 {
        return Err("No data received".into());
    }

    Ok(req.wLenDone as usize)
}

fn main() {
    unsafe {
        if let Some(tree) = find_device_tree_by_name("Razer Basilisk V3 Pro") {
            println!("Found device tree for 'Razer Basilisk V3 Pro':");
            print_device_tree(&tree, 0);
            let amount_of_interfaces = find_all_interfaces(&tree);
            println!("Found {} interfaces", amount_of_interfaces.len());
            let iface0 = find_interface_by_number(&tree, 0);
            if let Some(iface) = iface0 {
                if let Some(class) = get_string_property(iface.entry_id.unwrap(), "IOClass") {
                    println!("→ entry IOClass: {}", class);
                }
                println!("Found interface 3: {:?}", iface);
                let report = RazerReport::get_firmware_report();
                let bytes = report.to_hid_bytes();
                let device_interface = get_usb_device_interface(iface.io_service).unwrap();
                if let Err(e) = send_feature_report(device_interface, &*bytes, 3, RAZER_NEW_MOUSE_RECEIVER_WAIT_MIN_US as u64, RAZER_NEW_MOUSE_RECEIVER_WAIT_MAX_US as u64) {
                    println!("❌ Failed to send feature report: {}", e);
                } else {
                    println!("✅ Feature report sent successfully.");
                }
            } else {
                println!("❌ Interface 3 not found.");
            }
        } else {
            println!("❌ Device not found.");
        }
    }
}