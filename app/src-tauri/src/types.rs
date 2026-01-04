#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum DeviceType {
    Mouse,
    Keyboard,
    Headset,
    Other,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Feature {
    RGB,
    RGBExtended,
    BatteryStatus,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub name: String,
    pub vendor_id: String,
    pub product_id: String,
    pub device_type: String,
    pub features: Vec<String>,
}

pub struct DeviceCollection {
    pub devices: Vec<DeviceInfo>,
}


impl DeviceCollection {
    pub fn new() -> Self {
        DeviceCollection {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: DeviceInfo) {
        self.devices.push(device);
    }

    pub fn get_devices(&self) -> &Vec<DeviceInfo> {
        &self.devices
    }
}