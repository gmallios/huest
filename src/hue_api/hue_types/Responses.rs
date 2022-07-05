use crate::{bridge::config_get_mac_addr, util::mac_addr_to_bridge_id, hue_api::config_model::BridgeConfig};
use serde::Serialize;


pub trait HueResponse {
    fn from_bridge_config(&self, bridge_config: BridgeConfig) -> String;
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
