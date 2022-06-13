use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Write},
    path::Path,
    sync::RwLock, os::unix::process,
};

use serde::{Deserialize, Serialize, de};

use super::device_model::Device;



pub fn load_devices() -> DeviceMap {
    let file = fs::read_to_string("config/Devices.yaml").unwrap();
    let device_list: DeviceMap = serde_yaml::from_str(&file).unwrap();
    return device_list;
}




pub fn load_bridge_config() -> BridgeConfig {
    // // Create bridge.yaml if it doesnt exist
    // if !Path::new("config/Bridge.yaml").exists() {
    //     let mut file = File::create("config/Bridge.yaml").unwrap();
    //     save_bridge_config(BridgeConfig::default()).unwrap();
    // }

    // let file = match fs::read_to_string("config/Bridge.yaml"){
    //     Ok(file) => file,
    //     Err(e) => {
    //         println!("fs::read_to_string error for bridge.yaml");
    //         std::process::exit(0);
    //     }
    // };

    // let bridge_config: BridgeConfig = match serde_yaml::from_str(&file){
    //     Ok(bridge_config) => bridge_config,
    //     Err(e) => {
    //         // Most likely model mismatch
    //         // Move bridge.yaml to bridge.yaml.bak.error
    //         // and create a new one
    //         println!("serde_yaml::from_str error for bridge.yaml");
    //         println!("Moving to bridge.yaml.bak.error and creating a new one"); 
    //         fs::rename("config/Bridge.yaml", "config/Bridge.yaml.bak.error").unwrap();
    //         save_bridge_config(BridgeConfig::default()).unwrap();
    //         BridgeConfig::default()
    //     }
    // };

    // return bridge_config;
    return crate::util::load_config::<BridgeConfig>(&"Bridge.yaml".to_string());
}

pub fn save_bridge_config(bridge_config: BridgeConfig) -> Result<(), std::io::Error> {
    let bridge_config_str = match serde_yaml::to_string(&bridge_config) {
        Ok(s) => s,
        Err(e) => {
            println!("Error: {}", e);
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Error: Failed to serialize bridge config",
            ));
        }
    };
    fs::write("config/Bridge.yaml", bridge_config_str)
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LinkButton {
    pub lastlinkbuttonpushed: u64,
    pub pressed: bool
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
        }
    }
}