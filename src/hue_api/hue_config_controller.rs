use std::{
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::Utc;
use log::info;
use mac_address::get_mac_address;
use uuid::{
    v1::{Context, Timestamp},
    Uuid,
};

// use crate::device_model::{DeviceMap, Device};
use crate::{
    util::{mac_addr_to_bridge_id, config::{create_config_dir_if_not_exists, load_config, save_config}}, hue_api::hue_types::Config::HueUser,
};

use super::{
    hue_types::{DeviceMap, Config::BridgeConfig, Device::DeviceItem},
};

#[derive(Clone)]
pub struct HueConfigControllerState {
    pub hue_config_controller: Arc<RwLock<HueConfigController>>,
}

impl HueConfigControllerState {
    // pub fn get_controller(&self) -> std::sync::MutexGuard<HueConfigController> {
    //     self.hue_config_controller.lock().unwrap()
    // }

    pub fn get_controller_read(&self) -> std::sync::RwLockReadGuard<HueConfigController> {
        self.hue_config_controller.read().unwrap()
    }

    pub fn get_controller_write(&self) -> std::sync::RwLockWriteGuard<HueConfigController> {
        self.hue_config_controller.write().unwrap()
    }

    // pub fn get_bridge_config(&self) -> &BridgeConfig {
    //     *self.get_controller().bridge_config
    // }
}

#[derive(Clone)]
pub struct HueConfigController {
    pub device_map: DeviceMap,
    pub device_array: Vec<DeviceItem>,
    pub bridge_config: BridgeConfig,
}

impl HueConfigController {
    pub fn new() -> HueConfigController {
        create_config_dir_if_not_exists().unwrap();
        let device_map = load_config::<DeviceMap>("Devices.yaml");
        let mut bridge_config = load_config::<BridgeConfig>("Bridge.yaml");

        // TODO: Proper error handling
        // TODO: Check for mac and override if not set/different, source of truth should be the mac of the selected interface.
        bridge_config.mac = get_mac_address().unwrap().unwrap().to_string();
        bridge_config.bridgeid = mac_addr_to_bridge_id(bridge_config.mac.as_str());
        bridge_config.linkbutton.pressed = false;

        HueConfigController {
            device_map: device_map,
            bridge_config: bridge_config,
            device_array: Vec::new(),
        }
    }

    pub fn save(&self) {
        save_config("Bridge.yaml", self.bridge_config.clone())
            .expect("Failed to save bridge config");
    }

    pub fn get_device_list(&self) -> DeviceMap {
        self.device_map.clone()
    }

    pub fn set_device_list(&mut self, device_list: DeviceMap) {
        self.device_map = device_list;
    }

    pub fn is_link_button_pressed(&self) -> bool {
        // TODO: Fix this mess
        // let unix_timestamp = SystemTime::now()
        //     .duration_since(UNIX_EPOCH)
        //     .unwrap()
        //     .as_secs();
        // let millis_ellapsed = unix_timestamp - &self.bridge_config.linkbutton.lastlinkbuttonpushed;

        // self.bridge_config.linkbutton.pressed = false;

        // if ((millis_ellapsed as i64) / 1000) <= 30 {
        //     self.bridge_config.linkbutton.pressed = true;
        // }
        self.bridge_config.linkbutton.pressed
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

    pub fn add_user(&mut self, devicetype: &str, generate_client_key: &Option<bool>) -> (String, Option<String>) {
        // let context = Context::new(rand::random::<u16>());
        // let ts = Timestamp::from_unix(&context, Utc::now().to_, None);
        // let uuid = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6])
        //     .to_string()
        //     .replace("-", "");
        let uuid = Uuid::new_v4().to_string().replace("-", "");

        let mut keys = self
            .bridge_config
            .hue_users
            .clone()
            .into_keys()
            .collect::<Vec<u8>>();

        keys.sort();
        
        //info!("{:?}", keys);
        let fkey = match keys.last() {
            Some(k) => {
                //info!("key {}", k);
                k + 1
            }
            None => 0,
        };

        //info!("key: {:?}", fkey);

        self.bridge_config.hue_users.insert(
            fkey,
            HueUser {
                client_key: uuid.clone(),
                devicetype: devicetype.to_string(),
                date_created: Utc::now().timestamp().to_string(),
                date_last_connected: Utc::now().timestamp().to_string(),
            },
        );

        self.save();

        match generate_client_key {
            Some(true) => (uuid, Some(Uuid::new_v4().to_string().replace("-", ""))),
            _ => (uuid, None),
        }
    }

    pub fn user_exists(&self, client_key: &str) -> bool {
        self.bridge_config.clone().hue_users.into_values().any(|user| user.client_key == client_key)
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
