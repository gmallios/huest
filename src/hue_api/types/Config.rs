use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

//https://crates.io/crates/serde_with for more convenient types

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
    pub hue_users: BTreeMap<u8, HueUser>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LinkButton {
    pub lastlinkbuttonpushed: i64,
    pub pressed: bool,
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
            name: String::from("Huest Bridge"),
            swversion: String::new(),
            apiversion: String::new(),
            mac: String::new(),
            ipaddress: String::new(),
            gateway: String::new(),
            netmask: String::new(),
            bridgeid: String::new(),
            timezone: "Europe/London".to_string(),
            linkbutton: LinkButton {
                lastlinkbuttonpushed: 0,
                pressed: false,
            },
            hue_users: BTreeMap::new(),
        }
    }
}
