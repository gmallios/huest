use std::{clone, collections::HashMap};

use crate::{bridge::get_mac_addr, util::mac_addr_to_bridge_id};
use serde::{Deserialize, Serialize};

use super::{
    Config::{BridgeConfig, HueUser},
    Device::HueDeviceMap,
    Group::HueGroupMap,
};

pub trait HueResponse<T> {
    // Returns JSON stringified data
    fn from_bridge_config(
        bridge_config: BridgeConfig,
        _device_map: Option<HueDeviceMap>,
        _group_map: Option<HueGroupMap>,
    ) -> String;

    // Returns a new instance with data from bridge_config
    // Not required for everything so provide default implementation
    fn build_from_bconf(
        obj: &T,
        _bridge_config: BridgeConfig,
        _device_map: Option<HueDeviceMap>,
        _group_map: Option<HueGroupMap>,
    ) -> T
    where
        T: Clone,
    {
        return obj.clone();
    }
}

#[derive(Serialize, Default)]
pub struct DatastoreResponse {
    pub lights: HueDeviceMap,
    pub groups: HueGroupMap,
    pub config: HueConfigurationResponse,
}

impl HueResponse<DatastoreResponse> for DatastoreResponse {
    fn from_bridge_config(
        bridge_config: BridgeConfig,
        device_map: Option<HueDeviceMap>,
        groupmap: Option<HueGroupMap>,
    ) -> String {
        json!(DatastoreResponse {
            lights: device_map.unwrap(),
            groups: groupmap.unwrap(),
            config: HueConfigurationResponse::build_from_bconf(
                &HueConfigurationResponse::default(),
                bridge_config,
                None,
                None
            )
        })
        .to_string()
    }
}

// TODO: Remove???
#[derive(Serialize, Clone)]
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

pub fn hue_users_to_whitelist(
    hue_users: &std::collections::HashMap<u8, HueUser>,
) -> HashMap<String, Whitelist> {
    let mut whitelist: HashMap<String, Whitelist> = HashMap::new();
    for (_key, value) in hue_users {
        whitelist.insert(
            value.client_key.clone(),
            Whitelist {
                last_use_date: value.date_last_connected.clone(),
                create_date: value.date_created.clone(),
                name: value.devicetype.clone(),
            },
        );
    }
    return whitelist;
}

// TODO: Implement all params HueResponse for HueConfigurationResponse
impl HueResponse<HueConfigurationResponse> for HueConfigurationResponse {
    fn from_bridge_config(
        bridge_config: BridgeConfig,
        _device_map: Option<HueDeviceMap>,
        _groupmap: Option<HueGroupMap>,
    ) -> String {
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

    fn build_from_bconf(
        obj: &HueConfigurationResponse,
        bridge_config: BridgeConfig,
        _device_map: Option<HueDeviceMap>,
        _groupmap: Option<HueGroupMap>,
    ) -> HueConfigurationResponse {
        HueConfigurationResponse {
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
        }
    }
}

//TODO: Replace config_get_mac_addr
//TODO: Move to hue_types and implement HueResponse for HueConfigResponse
//TODO: Better naming
impl Default for HueConfigResponse {
    fn default() -> Self {
        HueConfigResponse {
            apiversion: "1.50.0".to_string(),
            bridgeid: mac_addr_to_bridge_id(&get_mac_addr()),
            datastoreversion: "103".to_string(),
            factorynew: false,
            mac: get_mac_addr(),
            modelid: "BSB002".to_string(),
            name: "Rustue".to_string(),
            replacesbridgeid: "".to_string(),
            starterkitid: "".to_string(),
            swversion: "1952086020".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Portalstate {
    pub signedon: bool,
    pub incoming: bool,
    pub outgoing: bool,
    pub communication: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Swupdate {
    pub updatestate: i64,
    pub url: String,
    pub text: String,
    pub notify: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Whitelist {
    #[serde(rename = "last use date")]
    pub last_use_date: String,
    #[serde(rename = "create date")]
    pub create_date: String,
    pub name: String,
}

// Default impl
impl Default for HueConfigurationResponse {
    fn default() -> HueConfigurationResponse {
        HueConfigurationResponse {
            name: "Huest Bridge".to_string(),
            zigbeechannel: 15,
            mac: "".to_owned(),
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
