use std::{fs, sync::RwLock};

use super::device_model::DeviceMap;

pub fn load_devices() -> DeviceMap {
    let file = fs::read_to_string("config/Devices.yaml").unwrap();
    let device_list: DeviceMap = serde_yaml::from_str(&file).unwrap();
    return device_list;
}

// lazy_static! {
//     static ref DEVICE_LIST: RwLock<devices::DeviceList> =  RwLock::new(load_devices());
// }
