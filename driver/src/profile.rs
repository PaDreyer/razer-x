use crate::device_capability::DeviceCapability;
use crate::device_type::DeviceType;

pub(crate) trait Profile {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;
    const DEVICE_TYPE: DeviceType;
    const VENDOR_ID: u16;
    const PRODUCT_ID: u16;
    const DEVICE_CAPABILITIES: &'static [DeviceCapability];
}