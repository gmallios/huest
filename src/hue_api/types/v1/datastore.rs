use serde::Serialize;

use crate::hue_api::types::Config::BridgeConfig;

use super::{
    configuration::HueV1ConfigurationResponse, group::HueV1GroupMapResponse,
    light::HueV1LightMapResponse,
};

#[derive(Serialize)]
pub struct HueV1DatastoreResponse {
    pub config: HueV1ConfigurationResponse,
    pub lights: HueV1LightMapResponse,
    pub groups: HueV1GroupMapResponse,
    /*
    scenes,
    rules,
    resourcelinks,
    schedules,
    sensors,
    */
}

impl HueV1DatastoreResponse {
    pub fn build(
        bridge_config: &BridgeConfig,
        lights: HueV1LightMapResponse,
        groups: HueV1GroupMapResponse,
    ) -> HueV1DatastoreResponse {
        HueV1DatastoreResponse {
            lights,
            groups,
            config: HueV1ConfigurationResponse::from(bridge_config),
        }
    }
}
