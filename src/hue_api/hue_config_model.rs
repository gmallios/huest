use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Write},
    path::Path,
    sync::RwLock, os::unix::process,
};

use serde::{Deserialize, Serialize, de};

use crate::util::save_config;

use super::device_model::Device;



pub fn load_devices() -> DeviceMap {
    let file = fs::read_to_string("config/Devices.yaml").unwrap();
    let device_list: DeviceMap = serde_yaml::from_str(&file).unwrap();
    return device_list;
}






pub fn save_device_config(device_map: DeviceMap) -> Result<(), std::io::Error> {
    let device_map_str = match serde_yaml::to_string(&device_map) {
        Ok(s) => s,
        Err(e) => {
            println!("Error: {}", e);
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Error: Failed to serialize device map",
            ));
        }
    };
    fs::write("config/Devices.yaml", device_map_str)
}

pub type DeviceMap = HashMap<u8, Device>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BridgeConfig {
    pub name: String,
    pub swversion: String,
    pub apiversion: String,
    pub mac: String,
    pub ipaddress: String,
    pub gateway: String,
    pub netmask: String,
    pub bridgeid: String,
    pub timezone: String,
    pub linkbutton: LinkButton,
    pub hue_users: HashMap<u8, HueUser>,
}
// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(untagged)]
// pub enum HueUsers{
//     HashMap(u8, HueUser)
// }


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LinkButton {
    pub lastlinkbuttonpushed: u64,
    pub pressed: bool
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HueUser {
    pub devicetype: String,
    pub client_key: String,
    pub date_created: String,
    pub date_last_connected: String,
}

// Implement defaults for BridgeConfig
impl Default for BridgeConfig {
    fn default() -> BridgeConfig {
        BridgeConfig {
            name: String::new(),
            swversion: String::new(),
            apiversion: String::new(),
            mac: String::new(),
            ipaddress: String::new(),
            gateway: String::new(),
            netmask: String::new(),
            bridgeid: String::new(),
            timezone: String::new(),
            linkbutton: LinkButton {
                lastlinkbuttonpushed: 0,
                pressed: false
            },
            hue_users: HashMap::new()
        }
    }
}