use serde::Serialize;

use crate::hue_api::types::Config::BridgeConfig;

use super::{
    configuration::HueV1ConfigurationResponse, group::HueV1GroupMapSimpleResponse,
    light::HueV1LightSimpleMapResponse,
};

#[derive(Serialize)]
pub struct HueV1DatastoreResponse {
    pub config: HueV1ConfigurationResponse,
    pub lights: HueV1LightSimpleMapResponse,
    pub groups: HueV1GroupMapSimpleResponse,
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
        lights: HueV1LightSimpleMapResponse,
        groups: HueV1GroupMapSimpleResponse,
    ) -> HueV1DatastoreResponse {
        HueV1DatastoreResponse {
            lights,
            groups,
            config: HueV1ConfigurationResponse::from(bridge_config),
        }
    }
}
