use serde::Serialize;

use crate::hue_api::types::{Config::BridgeConfig};

use super::{configuration::HueV1ConfigurationResponse, responses::HueV1Response, light::HueV1LightMapResponse, group::HueV1GroupMapResponse};

#[derive(Serialize)]
pub struct HueV1DatastoreResponse {
    pub lights: HueV1LightMapResponse,
    pub groups: HueV1GroupMapResponse,
    pub config: HueV1ConfigurationResponse,
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
            config: HueV1ConfigurationResponse::from(bridge_config)
        }
    }
}
