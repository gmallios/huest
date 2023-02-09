use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::hue_api::{devices::LightDevice, types::internal::ModelIDs};

#[derive(Serialize, Deserialize, Debug)]
pub struct HueV1LightSimpleMapResponse(BTreeMap<u8, HueV1LightSimpleItemResponse>);
#[derive(Serialize, Deserialize, Debug)]
pub struct HueV1LightMapResponse(BTreeMap<u8, HueV1LightItemResponse>);

impl HueV1LightMapResponse {
    pub async fn build(devices: &BTreeMap<u8, Box<dyn LightDevice>>) -> HueV1LightMapResponse {
        let mut lights = BTreeMap::new();
        for (_id, device) in devices {
            lights.insert(device.get_v1_id(), device.get_v1_state().await);
        }
        Self(lights)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HueV1LightItemResponse {
    pub name: String,
    pub modelid: ModelIDs,
    pub swversion: String,
    pub state: State,
    #[serde(rename = "type")]
    pub ltype: String,
    pub swupdate: Swupdate,
    pub capabilities: Capabilities,
}

impl HueV1LightSimpleMapResponse {
    pub async fn build(
        devices: &BTreeMap<u8, Box<dyn LightDevice>>,
    ) -> HueV1LightSimpleMapResponse {
        let mut lights = BTreeMap::new();
        for (_id, device) in devices {
            lights.insert(device.get_v1_id(), device.get_v1_state_simple().await);
        }
        Self(lights)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelIDData {
    // pub v1: ModelIDV1Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelIDV1Data {
    #[serde(rename = "type")]
    pub ltype: String,
    pub manufacturername: String,
    pub swversion: String,
    pub swupdate: Swupdate,
    pub capabilities: Capabilities,
}

impl ModelIDData {
    pub fn LCT001_V1() -> ModelIDV1Data {
        ModelIDV1Data {
            ltype: "Extended color light".to_string(),
            manufacturername: "Philips".to_string(),
            swversion: "1.90.0".to_string(),
            swupdate: Swupdate {
                state: "noupdates".to_string(),
                lastinstall: "2021-01-01T00:00:00".to_string(),
            },
            capabilities: Capabilities {
                certified: true,
                control: Control {
                    mindimlevel: 5000,
                    maxlumen: 600,
                    colorgamuttype: "B".to_string(),
                    colorgamut: vec![vec![0.675, 0.322], vec![0.409, 0.518], vec![0.167, 0.04]],
                    ct: Ct { min: 153, max: 500 },
                },
                streaming: Streaming {
                    renderer: false,
                    proxy: false,
                },
            },
        }
    }

    pub fn LST002_V1() -> ModelIDV1Data {
        ModelIDV1Data {
            ltype: "Color light".to_string(),
            manufacturername: "Signify Netherlands B.V.".to_string(),
            swversion: "1.90.1".to_string(),
            swupdate: Swupdate {
                state: "noupdates".to_string(),
                lastinstall: "2020-12-09T19:13:52".to_string(),
            },
            capabilities: Capabilities {
                certified: true,
                control: Control {
                    colorgamut: vec![vec![0.6915, 0.3083], vec![0.17, 0.7], vec![0.1532, 0.0475]],
                    colorgamuttype: "C".to_string(),
                    ct: Ct { max: 153, min: 500 },
                    maxlumen: 1600,
                    mindimlevel: 40,
                },
                streaming: Streaming {
                    renderer: true,
                    proxy: true,
                },
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HueV1LightSimpleItemResponse {
    pub state: State,
    #[serde(rename = "type")]
    pub ltype: String,
    pub name: String,
    pub modelid: ModelIDs,
    pub swversion: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ct {
    pub max: i64,
    pub min: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Swupdate {
    pub state: String,
    pub lastinstall: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Capabilities {
    pub certified: bool,
    pub control: Control,
    pub streaming: Streaming,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Streaming {
    pub renderer: bool,
    pub proxy: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Control {
    pub colorgamut: Vec<Vec<f64>>,
    pub colorgamuttype: String,
    pub ct: Ct,
    pub maxlumen: i64,
    pub mindimlevel: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    archetype: String,
    function: String,
    direction: String,
    startup: Startup,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Startup {
    mode: String,
    configured: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {
    pub on: bool,
    pub bri: i64,
    pub hue: i64,
    pub sat: i64,
    pub xy: (f32, f32),
    pub ct: i64,
    pub alert: String,
    pub mode: String,
    pub effect: String,
    pub colormode: String,
    pub reachable: bool,
}
