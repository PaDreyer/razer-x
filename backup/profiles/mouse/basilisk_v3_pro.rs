use crate::profile::Profile;
use crate::device_capability::{ DeviceCapability, MouseCapability };
use crate::device_type::DeviceType;

pub struct BasiliskV3Pro;

impl Profile for BasiliskV3Pro {
    const NAME: &'static str = "Basilisk V3 Pro";
    const DESCRIPTION: &'static str = "Razer Basilisk V3 Pro";
    const DEVICE_TYPE: DeviceType = DeviceType::Mouse;
    const VENDOR_ID: u16 = 0x1532;
    const PRODUCT_ID: u16 = 0x0056;
    const DEVICE_CAPABILITIES: &'static [DeviceCapability] = &[
        DeviceCapability::MouseCapability(
            MouseCapability::DPI
        ),
        DeviceCapability::MouseCapability(
            MouseCapability::PollingRate
        ),
        DeviceCapability::MouseCapability(
            MouseCapability::RGB,
        ),
        DeviceCapability::MouseCapability(
            MouseCapability::Brightness,
        )
    ];
}