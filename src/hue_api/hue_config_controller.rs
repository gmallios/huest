use std::sync::{RwLock, Arc, Mutex};

// use crate::device_model::{DeviceMap, Device};
use crate::hue_api::hue_config_model::{load_devices, load_bridge_config};

use super::{device_model::{Device}, hue_config_model::{BridgeConfig, DeviceMap}};


// lazy_static!{
//     pub static ref HUE_CONFIG_CONTROLLER: Arc<Mutex<HueConfigController>> = Arc::new(Mutex::new(HueConfigController::new()));
// }


pub struct HueConfigControllerState {
    pub hue_config_controller: Arc<Mutex<HueConfigController>>,
}

#[derive(Clone)]
pub struct HueConfigController {
    device_map: DeviceMap,
    device_array: Vec<Device>,
    bridge_config: BridgeConfig
}



impl HueConfigController {

    pub fn new() -> HueConfigController {
        println!("hueconfigcontroller init");
        let device_map = load_devices();
        let bridge_config = load_bridge_config();

        HueConfigController {
            device_map: device_map,
            bridge_config: bridge_config,
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

