use std::{fs::{self, File}, sync::RwLock, collections::HashMap, io::{self, Write}, path::Path};

use serde::{Deserialize, Serialize};

use super::device_model::Device;





pub fn load_devices() -> DeviceMap {
    let file = fs::read_to_string("config/Devices.yaml").unwrap();
    let device_list: DeviceMap = serde_yaml::from_str(&file).unwrap();
    return device_list;
}

pub fn load_bridge_config() -> BridgeConfig {
    // Create bridge.yaml if it doesnt exist
    if !Path::new("config/Bridge.yaml").exists() {
        let mut file = File::create("config/Bridge.yaml").unwrap();
        let bconf = BridgeConfig {
            swversion: "".to_string(),
            apiversion: "".to_string(),
            mac: "".to_string(),
            netmask: "".to_string(),
            gateway: "".to_string(),
            timezone: "".to_string(),
            bridgeid: "".to_string(),
            name: "".to_string(),
            ipaddress: "".to_string(),
        };
       fs::write("config/Bridge.yaml", serde_yaml::to_string(&bconf).unwrap()).unwrap();
    }

    let file = fs::read_to_string("config/Bridge.yaml").unwrap();
    let bridge_config: BridgeConfig = serde_yaml::from_str(&file).unwrap();
    return bridge_config;
}

pub fn save_bridge_config(bridge_config: BridgeConfig) -> Result<(), std::io::Error>{
    let bridge_config_str = match serde_yaml::to_string(&bridge_config) {
        Ok(s) => s,
        Err(e) => {
            println!("Error: {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, "Error: Failed to serialize bridge config"));
        }
    };
    fs::write("config/Bridge.yaml", bridge_config_str)
}


pub fn save_device_config(device_map: DeviceMap) -> Result<(), std::io::Error>{
    let device_map_str = match serde_yaml::to_string(&device_map) {
        Ok(s) => s,
        Err(e) => {
            println!("Error: {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, "Error: Failed to serialize device map"));
        }
    };
    fs::write("config/Devices.yaml", device_map_str)
}



pub type DeviceMap = HashMap<u8, Device>;

#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct BridgeConfig {
    name: String,
    swversion: String,
    apiversion: String,
    mac: String,
    ipaddress: String,
    gateway: String,
    netmask: String,
    bridgeid: String,
    timezone: String,
    linkbutton: LinkButton

}

#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct LinkButton{
    pub lastlinkbuttonpushed: String
}

// pub enum ModelIDs {

// }


// lazy_static! {
//     static ref DEVICE_LIST: RwLock<devices::DeviceList> =  RwLock::new(load_devices());
// }
