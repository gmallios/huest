use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use crate::{
    bridge::{get_gateway_ip, get_local_ip, get_mac_addr, get_netmask},
    hue_api::types::Config::HueUser,
    util::{
        config::{create_config_dir_if_not_exists, load_config, save_config},
        mac_addr_to_bridge_id,
    },
};
use chrono::Utc;
use uuid::Uuid;

use super::{
    devices::{wled::WLEDDevice, LightDevice},
    types::{
        internal::{DeviceProtos, GroupInstance, InternalDeviceMap, InternalGroupMap},
        Config::BridgeConfig,
    },
};

pub type GroupInstances = Arc<RwLock<BTreeMap<u8, GroupInstance>>>;
pub type LightInstances = Arc<RwLock<BTreeMap<u8, Box<dyn LightDevice>>>>;

pub struct HueConfigController {
    pub bridge_config: BridgeConfig,
    device_client: reqwest::Client,
    // InternalDevice/Group Maps are used on load/save
    // They are converted to Instance Maps on load and back to InternalDevice/Group Maps on save
    pub internal_device_map: InternalDeviceMap,
    pub internal_group_map: InternalGroupMap,
    // Instance Maps are used for runtime operations, they are populated from the Device/Group Maps
    // and are converted to InternalDevice/Group Maps on save
    pub light_instances: LightInstances,
    pub group_instances: GroupInstances,
}

impl HueConfigController {
    pub async fn new() -> HueConfigController {
        create_config_dir_if_not_exists().expect("Could not create config directory.");

        // Create Group 0
        let a = super::types::internal::InternalGroup {
            name: "Group 0".into(),
            id_v2: "idkyet".into(),
            lights: vec![0],
            state: super::types::internal::InternalGroupState::default(),
        };

        let mut internal_device_map = load_config::<InternalDeviceMap>("Devices.yaml");
        let mut group_map = load_config::<InternalGroupMap>("Groups.yaml");
        group_map.0.insert(0, a);
        let bridge_config = Self::init_bridge_config(load_config::<BridgeConfig>("Bridge.yaml"));

        let devices: LightInstances = Arc::new(RwLock::new(BTreeMap::new()));
        let mut group_instances: GroupInstances = Arc::new(RwLock::new(BTreeMap::new()));

        let device_client = reqwest::Client::new();

        // TODO: We should be able to rebuild the device BTreeMap at runtime
        //       since we need to push newly discovered devices to it.
        for (id, mut device) in internal_device_map.iter_mut() {
            /* We set the id_v1 since it's not deserialized and corresponds to the key of the device */
            device.id_v1 = *id;
            match device.proto {
                DeviceProtos::WLED => {
                    devices.write().unwrap().insert(
                        *id,
                        Box::new(WLEDDevice::new(device, device_client.clone()).await),
                    );
                }
            }
        }

        

        for (group_id, group) in group_map.0.iter() {
            group_instances
                .write()
                .unwrap()
                .insert(*group_id, GroupInstance::new(group, devices.clone()));
        }

        HueConfigController {
            internal_device_map,
            internal_group_map: group_map,
            bridge_config,
            light_instances: devices,
            device_client,
            group_instances,
        }
    }

    fn init_bridge_config(mut bridge_config: BridgeConfig) -> BridgeConfig {
        bridge_config.mac = get_mac_addr();
        bridge_config.bridgeid = mac_addr_to_bridge_id(bridge_config.mac.as_str());
        bridge_config.linkbutton.pressed = false;
        bridge_config.gateway = get_gateway_ip();
        bridge_config.netmask = get_netmask();
        bridge_config.ipaddress = get_local_ip();
        bridge_config
    }

    pub fn save(&self) {
        // TODO: Log error if save fails
        save_config("Bridge.yaml", &self.bridge_config).expect("Failed to save bridge config");
        save_config("Devices.yaml", &self.internal_device_map).expect("Failed to save device map");
    }

    pub fn is_link_button_pressed(&mut self) -> bool {
        let timestamp = Utc::now().timestamp();
        let millis_ellapsed = timestamp - &self.bridge_config.linkbutton.lastlinkbuttonpushed;

        self.bridge_config.linkbutton.pressed = false;

        if millis_ellapsed <= 30 {
            self.bridge_config.linkbutton.pressed = true;
        }
        self.bridge_config.linkbutton.pressed
    }

    pub fn press_link_button(&mut self) {
        let timestamp = Utc::now().timestamp();
        self.bridge_config.linkbutton.lastlinkbuttonpushed = timestamp;
        self.bridge_config.linkbutton.pressed = true;
    }

    pub fn add_user(
        &mut self,
        devicetype: &str,
        generate_client_key: &Option<bool>,
    ) -> (String, Option<String>) {
        // let context = Context::new(rand::random::<u16>());
        // let ts = Timestamp::from_unix(&context, Utc::now().to_, None);
        // let uuid = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6])
        //     .to_string()
        //     .replace("-", "");
        let uuid = Uuid::new_v4().to_string().replace('-', "");

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
            Some(true) => (uuid, Some(Uuid::new_v4().to_string().replace('-', ""))),
            _ => (uuid, None),
        }
    }

    pub fn user_exists(&self, client_key: &str) -> bool {
        self.bridge_config
            .clone()
            .hue_users
            .into_values()
            .any(|user| user.client_key == client_key)
    }

    pub fn get_user_name(&self, client_key: &str) -> Option<String> {
        self.bridge_config
            .clone()
            .hue_users
            .into_values()
            .find(|user| user.client_key == client_key)
            .map(|user| user.devicetype)
    }

    pub fn update_swversion(&mut self, swversion: &str) {
        self.bridge_config.swversion = swversion.to_string();
        self.save();
    }
}
