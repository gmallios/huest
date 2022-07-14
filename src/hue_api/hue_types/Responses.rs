use std::collections::HashMap;

use crate::{bridge::config_get_mac_addr, util::mac_addr_to_bridge_id};
use serde::Serialize;

use super::{Whitelist, HueConfigurationResponse, Config::{BridgeConfig, HueUser}};


pub trait HueResponse {
    fn from_bridge_config(&self, bridge_config: BridgeConfig) -> String;
}


#[derive(Serialize,Default)]
pub struct DatastoreResponse {
    pub config: HueConfigurationResponse
}

// TODO: Remove???
#[derive(Serialize)]
pub struct HueConfigResponse {
    pub apiversion: String,
    pub bridgeid: String,
    pub datastoreversion: String,
    pub factorynew: bool,
    pub mac: String,
    pub modelid: String,
    pub name: String,
    pub replacesbridgeid: String,
    pub starterkitid: String,
    pub swversion: String,
}


pub fn hue_users_to_whitelist(hue_users: &std::collections::HashMap<u8, HueUser>) -> HashMap<String, Whitelist> {
    let mut whitelist: HashMap<String, Whitelist> = HashMap::new();
    for (_key, value) in hue_users {
        whitelist.insert(value.client_key.clone(), super::Whitelist {
            last_use_date: value.date_last_connected.clone(),
            create_date: value.date_created.clone(),
            name: value.devicetype.clone(),
        });
    }
    return whitelist;
}

// TODO: Implement HueResponse for HueConfigurationResponse
impl HueResponse for HueConfigurationResponse {
    fn from_bridge_config(&self, bridge_config: BridgeConfig) -> String {
        json!(HueConfigurationResponse {
            mac: bridge_config.mac,
            name: bridge_config.name,
            ipaddress: bridge_config.ipaddress,
            netmask: bridge_config.netmask,
            gateway: bridge_config.gateway,
            timezone: bridge_config.timezone,
            swversion: bridge_config.swversion,
            apiversion: bridge_config.apiversion,
            whitelist: hue_users_to_whitelist(&bridge_config.hue_users),
            ..Default::default()
        })
        .to_string()
    }
}


//TODO: Replace config_get_mac_addr
//TODO: Move to hue_types and implement HueResponse for HueConfigResponse
//TODO: Better naming
impl Default for HueConfigResponse {
    fn default() -> Self {
        HueConfigResponse {
            apiversion: "1.50.0".to_string(),
            bridgeid: mac_addr_to_bridge_id(&config_get_mac_addr()),
            datastoreversion: "103".to_string(),
            factorynew: false,
            mac: config_get_mac_addr(),
            modelid: "BSB002".to_string(),
            name: "Rustue".to_string(),
            replacesbridgeid: "".to_string(),
            starterkitid: "".to_string(),
            swversion: "1950207110".to_string(),
        }
    }
}
