

use crate::{
    hue_api::types::{
        Config::{BridgeConfig},
    },
};



use serde::{Deserialize, Serialize};

use super::light::HueV1LightMapResponse;



pub trait HueV1Response<T> {
    // Returns JSON stringified data
    fn from_bridge_config(
        bridge_config: BridgeConfig,
        _device_map: Option<HueV1LightMapResponse>,
        // _group_map: Option<HueGroupMap>,
    ) -> String;

    // Returns a new instance with data from bridge_config
    // Not required for everything so provide default implementation
    fn build_from_bconf(
        obj: &T,
        _bridge_config: BridgeConfig,
        _device_map: Option<HueV1LightMapResponse>,
        // _group_map: Option<HueGroupMap>,
    ) -> T
    where
        T: Clone,
    {
        obj.clone()
    }
}



#[derive(Serialize, Clone)]
pub struct HueV1SmallConfigResponse {
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

impl From<&BridgeConfig> for HueV1SmallConfigResponse {
    fn from(value: &BridgeConfig) -> Self {
        HueV1SmallConfigResponse {
            apiversion: value.apiversion.to_owned(),
            bridgeid: value.bridgeid.to_owned(),
            datastoreversion: "103".to_owned(),
            factorynew: false,
            mac: value.mac.to_owned(),
            modelid: "BSB002".to_owned(),
            name: value.name.to_owned(),
            replacesbridgeid: "".to_owned(),
            starterkitid: "".to_owned(),
            swversion: value.swversion.to_owned(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BackupResponse {
    pub errorcode: i64,
    pub status: String,
}

impl Default for BackupResponse {
    fn default() -> Self {
        BackupResponse {
            errorcode: 0,
            status: "idle".to_string(),
        }
    }
}

