use std::{
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::Utc;
use mac_address::get_mac_address;
use uuid::{
    v1::{Context, Timestamp},
    Uuid,
};

// use crate::device_model::{DeviceMap, Device};
use crate::{
    hue_api::hue_config_model::{load_devices},
    util::{mac_addr_to_bridge_id, load_config, save_config},
};

use super::{
    device_model::Device,
    hue_config_model::{BridgeConfig, DeviceMap, HueUser},
};

// lazy_static!{
//     pub static ref HUE_CONFIG_CONTROLLER: Arc<Mutex<HueConfigController>> = Arc::new(Mutex::new(HueConfigController::new()));
// }

#[derive(Clone)]
pub struct HueConfigControllerState {
    pub hue_config_controller: Arc<Mutex<HueConfigController>>,
}

impl HueConfigControllerState {
    pub fn get_controller(&self) -> std::sync::MutexGuard<HueConfigController>  {
        self.hue_config_controller.lock().unwrap()
    }

    // pub fn get_bridge_config(&self) -> &BridgeConfig {
    //     *self.get_controller().bridge_config
    // }
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
        let mut bridge_config = load_config::<BridgeConfig>("Bridge.yaml");

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

    pub fn save(&self) {
        save_config("Bridge.yaml", self.bridge_config.clone()).expect("Failed to save bridge config");
    }

    pub fn get_device_list(&self) -> DeviceMap {
        return self.device_map.clone();
    }

    pub fn set_device_list(&mut self, device_list: DeviceMap) {
        self.device_map = device_list;
    }

    pub fn is_link_button_pressed(&mut self) -> bool {
        // let unix_timestamp = SystemTime::now()
        //     .duration_since(UNIX_EPOCH)
        //     .unwrap()
        //     .as_secs();
        // let millis_ellapsed = unix_timestamp - &self.bridge_config.linkbutton.lastlinkbuttonpushed;


        // self.bridge_config.linkbutton.pressed = false;

        // if ((millis_ellapsed as i64) / 1000) <= 30 {
        //     self.bridge_config.linkbutton.pressed = true;
        // }
        return self.bridge_config.linkbutton.pressed;
    }

    pub fn press_link_button(&mut self) {
        let unix_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.bridge_config.linkbutton.lastlinkbuttonpushed = unix_timestamp;
        self.bridge_config.linkbutton.pressed = true;
        println!("Link button pressed");
    }

    pub fn add_user(&mut self, devicetype: &str) -> String {
        let context = Context::new(rand::random::<u16>());
        let ts = Timestamp::from_unix(&context, 1497624119, 1234);
        let uuid = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6]).to_string().replace("-", "");
        let key: u8 = *self
            .bridge_config
            .hue_users
            .clone()
            .into_keys()
            .collect::<Vec<u8>>()
            .last()
            .unwrap_or_else(|| &0);

        self.bridge_config.hue_users.insert(
            key,
            HueUser {
                client_key: uuid.clone(),
                devicetype: devicetype.to_string(),
                date_created: Utc::now().timestamp().to_string(),
                date_last_connected: Utc::now().timestamp().to_string(),
            },
        );

        self.save();

        return uuid;
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
