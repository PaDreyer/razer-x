pub use core_foundation_sys::preferences::{
    CFPreferencesSetValue,
    CFPreferencesAppSynchronize,
    CFPreferencesSynchronize,
    CFPreferencesSetAppValue,
    CFPreferencesCopyAppValue,
    CFPreferencesGetAppBooleanValue,
    kCFPreferencesCurrentUser,
    kCFPreferencesAnyHost,
    kCFPreferencesAnyApplication,
};
pub use core_foundation::base::{TCFType, CFTypeRef};
pub use core_foundation::boolean::CFBoolean;
pub use core_foundation_sys::{
    base::{kCFAllocatorDefault, CFRetain},
    number::{CFNumberGetValue, kCFNumberSInt32Type},
    string::{CFStringGetCString, kCFStringEncodingUTF8},
    uuid::{CFUUIDCreateFromUUIDBytes}
};
pub use core_foundation_sys::uuid::CFUUIDRef;
pub use core_foundation::string::CFString;

// These are needed by the generated bindings
pub use core_foundation::string::CFStringRef;
pub use core_foundation_sys::base::CFAllocatorRef;

pub use core_foundation_sys::runloop::{
    CFRunLoopSourceRef,
    CFRunLoopRef,
    kCFRunLoopDefaultMode,
};
pub use core_foundation::uuid::{
    CFUUIDBytes,
    CFUUIDGetUUIDBytes,
};
pub use core_foundation_sys::array::CFArrayRef;
pub use core_foundation_sys::base::{CFIndex, CFOptionFlags};
pub use core_foundation::runloop::{
    CFRunLoopMode, CFRunLoopRunResult, CFRunLoopObserverRef, CFRunLoopTimerRef,
    CFRunLoopSourceContext, CFRunLoopObserverCallBack, CFRunLoopTimerContext,
    CFRunLoopTimerCallBack, CFRunLoopObserverContext
};
pub use core_foundation_sys::mach_port::CFTypeID;
pub use core_foundation_sys::date::{CFAbsoluteTime, CFTimeInterval};
pub use core_foundation_sys::dictionary::{CFDictionaryRef, CFMutableDictionaryRef};

use std::os::raw::c_int;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[allow(non_camel_case_types)]
pub type REFIID = CFUUIDBytes;

unsafe extern "C" {
    pub static kIOUSBDeviceUserClientTypeID: CFUUIDRef;
    pub static kIOCFPlugInInterfaceID: CFUUIDRef;
    pub static kIOUSBDeviceInterfaceID: CFUUIDRef;
}

const SYS_IOKIT: c_int = ((0x38) & 0x3f) << 26;
const SUB_IOKIT_COMMON: c_int = ((0) & 0xfff) << 14;

macro_rules! iokit_err {
    ($id:ident, $offset:expr) => {
        pub const $id: IOReturn = SYS_IOKIT | SUB_IOKIT_COMMON | $offset;
    };
}

pub const kIOReturnSuccess: IOReturn = KERN_SUCCESS as c_int;

iokit_err!(kIOReturnError, 0x2bc);
iokit_err!(kIOReturnNoMemory, 0x2bd);
iokit_err!(kIOReturnNoResources, 0x2be);
iokit_err!(kIOReturnIPCError, 0x2bf);
iokit_err!(kIOReturnNoDevice, 0x2c0);
iokit_err!(kIOReturnNotPrivileged, 0x2c1);
iokit_err!(kIOReturnBadArgument, 0x2c2);
iokit_err!(kIOReturnLockedRead, 0x2c3);
iokit_err!(kIOReturnLockedWrite, 0x2c4);
iokit_err!(kIOReturnExclusiveAccess, 0x2c5);
iokit_err!(kIOReturnBadMessageID, 0x2c6);
iokit_err!(kIOReturnUnsupported, 0x2c7);
iokit_err!(kIOReturnVMError, 0x2c8);
iokit_err!(kIOReturnInternalError, 0x2c9);
iokit_err!(kIOReturnIOError, 0x2ca);
iokit_err!(kIOReturnCannotLock, 0x2cc);
iokit_err!(kIOReturnNotOpen, 0x2cd);
iokit_err!(kIOReturnNotReadable, 0x2ce);
iokit_err!(kIOReturnNotWritable, 0x2cf);
iokit_err!(kIOReturnNotAligned, 0x2d0);
iokit_err!(kIOReturnBadMedia, 0x2d1);
iokit_err!(kIOReturnStillOpen, 0x2d2);
iokit_err!(kIOReturnRLDError, 0x2d3);
iokit_err!(kIOReturnDMAError, 0x2d4);
iokit_err!(kIOReturnBusy, 0x2d5);
iokit_err!(kIOReturnTimeout, 0x2d6);
iokit_err!(kIOReturnOffline, 0x2d7);
iokit_err!(kIOReturnNotReady, 0x2d8);
iokit_err!(kIOReturnNotAttached, 0x2d9);
iokit_err!(kIOReturnNoChannels, 0x2da);
iokit_err!(kIOReturnNoSpace, 0x2db);
iokit_err!(kIOReturnPortExists, 0x2dd);
iokit_err!(kIOReturnCannotWire, 0x2de);
iokit_err!(kIOReturnNoInterrupt, 0x2df);
iokit_err!(kIOReturnNoFrames, 0x2e0);
iokit_err!(kIOReturnMessageTooLarge, 0x2e1);
iokit_err!(kIOReturnNotPermitted, 0x2e2);
iokit_err!(kIOReturnNoPower, 0x2e3);
iokit_err!(kIOReturnNoMedia, 0x2e4);
iokit_err!(kIOReturnUnformattedMedia, 0x2e5);
iokit_err!(kIOReturnUnsupportedMode, 0x2e6);
iokit_err!(kIOReturnUnderrun, 0x2e7);
iokit_err!(kIOReturnOverrun, 0x2e8);
iokit_err!(kIOReturnDeviceError, 0x2e9);
iokit_err!(kIOReturnNoCompletion, 0x2ea);
iokit_err!(kIOReturnAborted, 0x2eb);
iokit_err!(kIOReturnNoBandwidth, 0x2ec);
iokit_err!(kIOReturnNotResponding, 0x2ed);
iokit_err!(kIOReturnIsoTooOld, 0x2ee);
iokit_err!(kIOReturnIsoTooNew, 0x2ef);
iokit_err!(kIOReturnNotFound, 0x2f0);
iokit_err!(kIOReturnInvalid, 0x1);
