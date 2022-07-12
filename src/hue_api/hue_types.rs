// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

pub mod Responses;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::config_model::BridgeConfig;

// JSON Response for whole config

#[derive(Serialize, Deserialize)]
pub struct HueConfigurationResponse {
    pub name: String,
    pub zigbeechannel: i64,
    pub mac: String,
    pub dhcp: bool,
    pub ipaddress: String,
    pub netmask: String,
    pub gateway: String,
    pub proxyaddress: String,
    pub proxyport: i64,
    #[serde(rename = "UTC")]
    pub utc: String,
    pub localtime: String,
    pub timezone: String,
    pub whitelist: HashMap<String, Whitelist>,
    pub swversion: String,
    pub apiversion: String,
    pub swupdate: Swupdate,
    pub linkbutton: bool,
    pub portalservices: bool,
    pub portalconnection: String,
    pub portalstate: Portalstate,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Portalstate {
    pub signedon: bool,
    pub incoming: bool,
    pub outgoing: bool,
    pub communication: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Swupdate {
    pub updatestate: i64,
    pub url: String,
    pub text: String,
    pub notify: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Whitelist {
    #[serde(rename = "last use date")]
    pub last_use_date: String,
    #[serde(rename = "create date")]
    pub create_date: String,
    pub name: String,
}


fn hue_users_to_whitelist(hue_users: &HashMap<u8, super::config_model::HueUser>) -> HashMap<String, Whitelist> {
    let mut whitelist: HashMap<String, Whitelist> = HashMap::new();
    for (key, value) in hue_users {
        whitelist.insert(value.client_key.clone(), Whitelist {
            last_use_date: value.date_last_connected.clone(),
            create_date: value.date_created.clone(),
            name: value.devicetype.clone(),
        });
    }
    return whitelist;
}

// TODO: Implement HueResponse for HueConfigurationResponse
impl Responses::HueResponse for HueConfigurationResponse {
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

// Default impl
impl Default for HueConfigurationResponse {
    fn default() -> HueConfigurationResponse {
        HueConfigurationResponse {
            name: "Huest Bridge".to_string(),
            zigbeechannel: 15,
            mac: "".to_string(),
            dhcp: false,
            ipaddress: "".to_string(),
            netmask: "".to_string(),
            gateway: "".to_string(),
            proxyaddress: "".to_string(),
            proxyport: 0,
            utc: "".to_string(),
            localtime: "".to_string(),
            timezone: "".to_string(),
            whitelist: HashMap::new(),
            swversion: "".to_string(),
            apiversion: "".to_string(),
            swupdate: Swupdate::default(),
            linkbutton: false,
            portalservices: false,
            portalconnection: "".to_string(),
            portalstate: Portalstate::default(),
        }
    }
}


