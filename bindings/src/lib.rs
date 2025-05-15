#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core_foundation_sys::uuid::CFUUIDRef;
// Those are excluded from the bindings, so we need to define them here
// Now they are compatible with the bindings
use core_foundation::string::CFString;
use core_foundation::string::CFStringRef;
use core_foundation_sys::base::CFTypeRef;
use core_foundation_sys::base::CFAllocatorRef;
use core_foundation_sys::runloop::CFRunLoopSourceRef;
use core_foundation_sys::dictionary::{
    CFDictionaryRef,
    CFMutableDictionaryRef
};
use core_foundation::uuid::CFUUIDBytes;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[allow(non_camel_case_types)]
pub type REFIID = CFUUIDRef;