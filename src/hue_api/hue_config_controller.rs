use std::{
    sync::{Arc, Mutex, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

use mac_address::get_mac_address;

// use crate::device_model::{DeviceMap, Device};
use crate::{
    hue_api::hue_config_model::{load_bridge_config, load_devices},
    util::mac_addr_to_bridge_id,
};

use super::{
    device_model::Device,
    hue_config_model::{BridgeConfig, DeviceMap},
};

// lazy_static!{
//     pub static ref HUE_CONFIG_CONTROLLER: Arc<Mutex<HueConfigController>> = Arc::new(Mutex::new(HueConfigController::new()));
// }

pub struct HueConfigControllerState {
    pub hue_config_controller: Arc<Mutex<HueConfigController>>,
}

#[derive(Clone)]
pub struct HueConfigController {
    pub device_map: DeviceMap,
    pub device_array: Vec<Device>,
    pub bridge_config: BridgeConfig,
}

impl HueConfigController {
    pub fn new() -> HueConfigController {
        println!("hueconfigcontroller init");
        let device_map = load_devices();
        let mut bridge_config = load_bridge_config();

        // TODO: Proper error handling
        // TODO: Check for mac and override if not set/different, source of truth should be get_mac_address()
        bridge_config.mac = get_mac_address().unwrap().unwrap().to_string();
        bridge_config.bridgeid = mac_addr_to_bridge_id(bridge_config.mac.as_str());

        HueConfigController {
            device_map: device_map,
            bridge_config: bridge_config,
            device_array: Vec::new(),
        }
    }

    pub fn get_device_list(&self) -> DeviceMap {
        return self.device_map.clone();
    }

    pub fn set_device_list(&mut self, device_list: DeviceMap) {
        self.device_map = device_list;
    }

    pub fn is_link_button_pressed(mut self) -> bool {
        let unix_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let millis_ellapsed = unix_timestamp - self.bridge_config.linkbutton.lastlinkbuttonpushed;

        if ((millis_ellapsed as i64) / 1000) <= 30 {
            self.bridge_config.linkbutton.pressed = true;
        }

        self.bridge_config.linkbutton.pressed = false;
        return self.bridge_config.linkbutton.pressed;
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
