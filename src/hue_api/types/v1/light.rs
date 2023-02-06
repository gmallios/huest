use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use crate::hue_api::{devices::LightDevice, types::internal::InternalDevice};

#[derive(Serialize, Deserialize, Debug)]
pub struct HueV1LightSimpleMapResponse(BTreeMap<u8, HueV1LightSimpleItemResponse>);
#[derive(Serialize, Deserialize, Debug)]
pub struct HueV1LightMapResponse(BTreeMap<u8, HueV1LightItemResponse>);

impl HueV1LightMapResponse {
    pub fn build(devices: &BTreeMap<u8, Box<dyn LightDevice>>) -> HueV1LightMapResponse {
        let mut lights = BTreeMap::new();
        for (_id, device) in devices {
            lights.insert(device.get_v1_id(), device.get_v1_state());
        }
        Self(lights)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HueV1LightItemResponse {
    pub name: String,
    pub modelid: String,
    pub swversion: String,
    #[serde(rename = "type")]
    pub ltype: String,
    pub state: State,
    pub swupdate: Swupdate,
    pub capabilities: Capabilities,
}

impl HueV1LightSimpleMapResponse {
    pub fn build(devices: &BTreeMap<u8, Box<dyn LightDevice>>) -> HueV1LightSimpleMapResponse {
        let mut lights = BTreeMap::new();
        for (_id, device) in devices {
            lights.insert(device.get_v1_id(), device.get_v1_state_simple());
        }
        Self(lights)
    }
}

/* v1_static */
pub struct ModelidV1Data {
    pub ltype: String,
    pub manufacturername: String,
    pub swversion: String,
    swupdate: Swupdate,
    capabilities: Capabilities,
}

// pub static V1MODELID_DATAMAP: Lazy<phf::Map<&str, ModelidV1Data>> = Lazy::new(|| { phf::phf_map! {
//     "LCT001" => ModelidV1Data {
//         ltype: "Extended color light".to_string(),
//         manufacturername: "Philips".to_string(),
//         swversion: "1.90.0".to_string(),
//         swupdate: Swupdate {
//             state: "noupdates".to_string(),
//             lastinstall: "2021-01-01T00:00:00".to_string(),
//         },
//         capabilities: Capabilities {
//             certified: true,
//             control: Control {
//                 mindimlevel: 5000,
//                 maxlumen: 600,
//                 colorgamuttype: "B".to_string(),
//                 colorgamut: vec![
//                     vec![0.675, 0.322],
//                     vec![0.409, 0.518],
//                     vec![0.167, 0.04],
//                 ],
//                 ct: Ct {
//                     min: 153,
//                     max: 500,
//                 },
//             },
//             streaming: Streaming {
//                 renderer: false,
//                 proxy: false,
//             },
//         },
//     },
// }};

pub static MODELID_DATAMAP_V1: Lazy<HashMap<&str, ModelidV1Data>> = Lazy::new(|| {
    let mut map: HashMap<&str, ModelidV1Data> = HashMap::new();
    map.insert(
        "LCT001",
        ModelidV1Data {
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
        },
    );
    map
});

#[derive(Serialize, Deserialize, Debug)]
pub struct HueV1LightSimpleItemResponse {
    pub name: String,
    pub modelid: String,
    pub swversion: String,
    #[serde(rename = "type")]
    pub ltype: String,
    //pub state: State,
    swupdate: Swupdate,
    capabilities: Capabilities,
}

impl From<&InternalDevice> for HueV1LightSimpleItemResponse {
    fn from(device: &InternalDevice) -> Self {
        let modelid = device.modelid.to_string();
        let v1_statics = MODELID_DATAMAP_V1.get(&modelid as &str).unwrap();
        HueV1LightSimpleItemResponse {
            name: device.name.clone(),
            modelid,
            swversion: v1_statics.swversion.clone(),
            ltype: v1_statics.ltype.clone(),
            swupdate: v1_statics.swupdate.clone(),
            capabilities: v1_statics.capabilities.clone(),
            //state: todo!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ct {
    max: i64,
    min: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Swupdate {
    state: String,
    lastinstall: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Capabilities {
    pub certified: bool,
    pub control: Control,
    pub streaming: Streaming,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Streaming {
    renderer: bool,
    proxy: bool,
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
    on: bool,
    bri: i64,
    hue: i64,
    sat: i64,
    xy: Vec<f64>,
    ct: i64,
    alert: String,
    mode: String,
    effect: String,
    colormode: String,
    reachable: bool,
}
