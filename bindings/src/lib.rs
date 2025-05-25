#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[cfg(target_os = "macos")]
mod platform {
    pub use core_foundation::base::TCFType;
    pub use core_foundation_sys::{
        base::kCFAllocatorDefault,
        number::{CFNumberGetValue, kCFNumberSInt32Type},
        string::{CFStringGetCString, kCFStringEncodingUTF8},
        uuid::{CFUUIDCreateFromUUIDBytes}
    };
    pub use core_foundation_sys::uuid::CFUUIDRef;
    // Those are excluded from the bindings, so we need to define them here
    // Now they are compatible with the bindings
    pub use core_foundation::string::CFString;
    use core_foundation::string::CFStringRef;
    use core_foundation_sys::base::CFTypeRef;
    use core_foundation_sys::base::CFAllocatorRef;
    pub use core_foundation_sys::runloop::{
        CFRunLoopSourceRef,
        CFRunLoopRef,
    };
    use core_foundation_sys::dictionary::{
        CFDictionaryRef,
        CFMutableDictionaryRef
    };
    pub use core_foundation::uuid::{
        CFUUIDBytes,
        CFUUIDGetUUIDBytes,
    };
    use std::os::raw::c_int;
    use std::collections::HashMap;
    use std::sync::OnceLock;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    
    #[allow(non_camel_case_types)]
    pub type REFIID = CFUUIDBytes;

    unsafe extern "C" {
        pub static kIOUSBDeviceUserClientTypeID: CFUUIDRef;
        pub static kIOCFPlugInInterfaceID: CFUUIDRef;
        pub static kIOUSBDeviceInterfaceID: CFUUIDRef;
    }

    // sys_iokit
    const SYS_IOKIT: c_int = ((0x38) & 0x3f) << 26;
    const SUB_IOKIT_COMMON: c_int = ((0) & 0xfff) << 14;

    // IOReturn
    // Already defined
    //pub type IOReturn = kern_return_t;

    // OK
    pub const kIOReturnSuccess: IOReturn = KERN_SUCCESS as c_int;
    // general error
    pub const kIOReturnError: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2bc;
    // can't allocate memory
    pub const kIOReturnNoMemory: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2bd;
    // resource shortage
    pub const kIOReturnNoResources: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2be;
    // error during IPC
    pub const kIOReturnIPCError: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2bf;
    // no such device
    pub const kIOReturnNoDevice: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2c0;
    // privilege violation
    pub const kIOReturnNotPrivileged: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2c1;
    // invalid argument
    pub const kIOReturnBadArgument: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2c2;
    // device read locked
    pub const kIOReturnLockedRead: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2c3;
    // device write locked
    pub const kIOReturnLockedWrite: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2c4;
    // exclusive access and device already open
    pub const kIOReturnExclusiveAccess: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2c5;
    // sent/received messages had different msg_id
    pub const kIOReturnBadMessageID: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2c6;
    // unsupported function
    pub const kIOReturnUnsupported: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2c7;
    // misc. VM failure
    pub const kIOReturnVMError: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2c8;
    // internal error
    pub const kIOReturnInternalError: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2c9;
    // General I/O error
    pub const kIOReturnIOError: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2ca;
    // ???
    // pub const kIOReturn???Error: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2cb;
    // can't acquire lock
    pub const kIOReturnCannotLock: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2cc;
    // device not open
    pub const kIOReturnNotOpen: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2cd;
    // read not supported
    pub const kIOReturnNotReadable: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2ce;
    // write not supported
    pub const kIOReturnNotWritable: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2cf;
    // alignment error
    pub const kIOReturnNotAligned: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2d0;
    // Media Error
    pub const kIOReturnBadMedia: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2d1;
    // device(s) still open
    pub const kIOReturnStillOpen: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2d2;
    // rld failure
    pub const kIOReturnRLDError: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2d3;
    // DMA failure
    pub const kIOReturnDMAError: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2d4;
    // Device Busy
    pub const kIOReturnBusy: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2d5;
    // I/O Timeout
    pub const kIOReturnTimeout: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2d6;
    // device offline
    pub const kIOReturnOffline: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2d7;
    // not ready
    pub const kIOReturnNotReady: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2d8;
    // device not attached
    pub const kIOReturnNotAttached: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2d9;
    // no DMA channels left
    pub const kIOReturnNoChannels: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2da;
    // no space for data
    pub const kIOReturnNoSpace: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2db;
    // ???
    // pub const kIOReturn???Error: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2dc;
    // port already exists
    pub const kIOReturnPortExists: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2dd;
    // can't wire down physical memory
    pub const kIOReturnCannotWire: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2de;
    // no interrupt attached
    pub const kIOReturnNoInterrupt: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2df;
    // no DMA frames enqueued
    pub const kIOReturnNoFrames: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2e0;
    // oversized msg received on interrupt port
    pub const kIOReturnMessageTooLarge: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2e1;
    // not permitted
    pub const kIOReturnNotPermitted: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2e2;
    // no power to device
    pub const kIOReturnNoPower: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2e3;
    // media not present
    pub const kIOReturnNoMedia: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2e4;
    // media not formatted
    pub const kIOReturnUnformattedMedia: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2e5;
    // no such mode
    pub const kIOReturnUnsupportedMode: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2e6;
    // data underrun
    pub const kIOReturnUnderrun: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2e7;
    // data overrun
    pub const kIOReturnOverrun: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2e8;
    // the device is not working properly!
    pub const kIOReturnDeviceError: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2e9;
    // a completion routine is required
    pub const kIOReturnNoCompletion: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2ea;
    // operation aborted
    pub const kIOReturnAborted: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2eb;
    // bus bandwidth would be exceeded
    pub const kIOReturnNoBandwidth: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2ec;
    // device not responding
    pub const kIOReturnNotResponding: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2ed;
    // isochronous I/O request for distant past!
    pub const kIOReturnIsoTooOld: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2ee;
    // isochronous I/O request for distant future
    pub const kIOReturnIsoTooNew: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2ef;
    // data was not found
    pub const kIOReturnNotFound: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2f0;
    // should never be seen
    pub const kIOReturnInvalid: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | 0x1;


    const K_SUCCESS: IOReturn = kIOReturnSuccess;

    pub fn log_ior_error(context: &str, code: IOReturn) {
        if code == K_SUCCESS {
            return;
        }

        let name = IOKIT_ERRORS
            .get()
            .unwrap()
            .get(&code)
            .map(|s| *s)
            .unwrap_or("Unknown");

        println!("❌ {} failed: {} (0x{:08x})", context, name, code as u32);
    }

    static IOKIT_ERRORS: OnceLock<HashMap<IOReturn, &'static str>> = OnceLock::new();

    pub fn init_ior_errors() {
        let mut m = HashMap::new();
        m.insert(kIOReturnError, "kIOReturnError");
        m.insert(kIOReturnNoMemory, "kIOReturnNoMemory");
        m.insert(kIOReturnNoResources, "kIOReturnNoResources");
        m.insert(kIOReturnIPCError, "kIOReturnIPCError");
        m.insert(kIOReturnNoDevice, "kIOReturnNoDevice");
        m.insert(kIOReturnNotPrivileged, "kIOReturnNotPrivileged");
        m.insert(kIOReturnBadArgument, "kIOReturnBadArgument");
        m.insert(kIOReturnLockedRead, "kIOReturnLockedRead");
        m.insert(kIOReturnLockedWrite, "kIOReturnLockedWrite");
        m.insert(kIOReturnExclusiveAccess, "kIOReturnExclusiveAccess");
        m.insert(kIOReturnBadMessageID, "kIOReturnBadMessageID");
        m.insert(kIOReturnUnsupported, "kIOReturnUnsupported");
        m.insert(kIOReturnVMError, "kIOReturnVMError");
        m.insert(kIOReturnInternalError, "kIOReturnInternalError");
        m.insert(kIOReturnIOError, "kIOReturnIOError");
        m.insert(kIOReturnCannotLock, "kIOReturnCannotLock");
        m.insert(kIOReturnNotOpen, "kIOReturnNotOpen");
        m.insert(kIOReturnNotReadable, "kIOReturnNotReadable");
        m.insert(kIOReturnNotWritable, "kIOReturnNotWritable");
        m.insert(kIOReturnNotAligned, "kIOReturnNotAligned");
        m.insert(kIOReturnBadMedia, "kIOReturnBadMedia");
        m.insert(kIOReturnStillOpen, "kIOReturnStillOpen");
        m.insert(kIOReturnRLDError, "kIOReturnRLDError");
        m.insert(kIOReturnDMAError, "kIOReturnDMAError");
        m.insert(kIOReturnBusy, "kIOReturnBusy");
        m.insert(kIOReturnTimeout, "kIOReturnTimeout");
        m.insert(kIOReturnOffline, "kIOReturnOffline");
        m.insert(kIOReturnNotReady, "kIOReturnNotReady");
        m.insert(kIOReturnNotAttached, "kIOReturnNotAttached");
        m.insert(kIOReturnNoChannels, "kIOReturnNoChannels");
        m.insert(kIOReturnNoSpace, "kIOReturnNoSpace");
        m.insert(kIOReturnPortExists, "kIOReturnPortExists");
        m.insert(kIOReturnCannotWire, "kIOReturnCannotWire");
        m.insert(kIOReturnNoInterrupt, "kIOReturnNoInterrupt");
        m.insert(kIOReturnNoFrames, "kIOReturnNoFrames");
        m.insert(kIOReturnMessageTooLarge, "kIOReturnMessageTooLarge");
        m.insert(kIOReturnNotPermitted, "kIOReturnNotPermitted");
        m.insert(kIOReturnNoPower, "kIOReturnNoPower");
        m.insert(kIOReturnNoMedia, "kIOReturnNoMedia");
        m.insert(kIOReturnUnformattedMedia, "kIOReturnUnformattedMedia");
        m.insert(kIOReturnUnsupportedMode, "kIOReturnUnsupportedMode");
        m.insert(kIOReturnUnderrun, "kIOReturnUnderrun");
        m.insert(kIOReturnOverrun, "kIOReturnOverrun");
        m.insert(kIOReturnDeviceError, "kIOReturnDeviceError");
        m.insert(kIOReturnNoCompletion, "kIOReturnNoCompletion");
        m.insert(kIOReturnAborted, "kIOReturnAborted");
        m.insert(kIOReturnNoBandwidth, "kIOReturnNoBandwidth");
        m.insert(kIOReturnNotResponding, "kIOReturnNotResponding");
        m.insert(kIOReturnIsoTooOld, "kIOReturnIsoTooOld");
        m.insert(kIOReturnIsoTooNew, "kIOReturnIsoTooNew");
        m.insert(kIOReturnNotFound, "kIOReturnNotFound");
        m.insert(kIOReturnInvalid, "kIOReturnInvalid");

        IOKIT_ERRORS.set(m).ok();
    }
}

#[cfg(target_os = "linux")]
mod platform {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(target_os = "windows")]
mod platform {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use platform::*;