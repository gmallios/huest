use std::sync::{RwLock, Arc};

// use crate::device_model::{DeviceMap, Device};
use crate::hue_api::hue_config_model::load_devices;

use super::device_model::{DeviceMap, Device};


lazy_static!{
    pub static ref HUE_CONFIG_CONTROLLER: Arc<RwLock<HueConfigController>> = Arc::new(RwLock::new(HueConfigController::new()));
}

#[derive(Clone)]
pub struct HueConfigController {
    device_map: DeviceMap,
    device_array: Vec<Device>
}

impl HueConfigController {

    pub fn new() -> HueConfigController {
        println!("init");
        HueConfigController {
            device_map: load_devices(),
            device_array: Vec::new()
        }
        
    }

    pub fn get_device_list(&self) -> DeviceMap {
        return self.device_map.clone();
    }

    pub fn set_device_list(&mut self, device_list: DeviceMap) {
        self.device_map = device_list;
    }

    // pub fn get_device_by_id(&self, id: &str) -> Option<devices::Device> {
    //     for device in self.device_map.get_devices() {
    //         if device.get_id() == id {
    //             return Some(device.clone());
    //         }
    //     }
    //     return None;
    // }
}

