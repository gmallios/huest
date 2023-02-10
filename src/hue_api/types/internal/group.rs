use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};

use crate::hue_api::{
    config_controller::LightInstances,
    devices::LightDevice,
    types::v1::group::{
        HueV1GroupMapSimpleResponse, HueV1GroupResponse, HueV1GroupSimpleResponse, HueV1GroupState, HueV1GroupTypes,
    },
};

#[derive(Serialize, Deserialize, Default)]
pub struct InternalGroupMap(pub BTreeMap<u8, InternalGroup>);

// impl InternalGroupMap {
//     pub fn get_v1_simple(&self) -> HueV1GroupMapResponse {
//         let mut groups = HueV1GroupMapResponse::new();
//         for (id, group) in self.0.iter() {
//             groups.insert(*id, group.get_v1_simple());
//         }
//         groups
//     }
// }

// Used to represent the internal state of a group which can be use to response to both V1 and V2 API
//
// InternalGroup -> impls GroupInstance -> fn v1_resp - Group
//                                      -> fn v2_resp - Room/Zone/Grouped Light
#[derive(Serialize, Deserialize)]
pub struct InternalGroup {
    pub name: String,
    pub id_v2: String,
    pub lights: Vec<u8>, /* v1 IDs of lights */
    pub state: InternalGroupState,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InternalGroupState {
    pub on: bool,
    pub bri: u8,
    pub hue: u16,
    pub sat: u8,
    pub effect: String,
    pub xy: (f32, f32),
    pub ct: u16,
    pub alert: String,
    pub colormode: String,
}

pub struct GroupInstance {
    name: String,
    id_v1: String,
    id_v2: String,
    lights: Vec<u8>, /* v1 IDs of lights */
    state: InternalGroupState,
    light_instances: LightInstances,
}

impl GroupInstance {
    pub fn new(
        group: &InternalGroup,
        devices: Arc<RwLock<BTreeMap<u8, Box<dyn LightDevice>>>>,
    ) -> Self {
        Self {
            name: group.name.clone(),
            id_v1: group.id_v2.clone(),
            id_v2: group.id_v2.clone(),
            lights: group.lights.clone(),
            state: group.state.clone(),
            light_instances: devices,
        }
    }

    pub fn get_v1_state(&self) -> HueV1GroupResponse {
        let mut any_on = false;
        let mut all_on = true;
        let devices = self.light_instances.read().unwrap();
        for (id, light) in devices.iter() {
            if self.lights.contains(id) {
                if light.is_on() {
                    any_on = true;
                } else {
                    all_on = false;
                }
            }
        }
        HueV1GroupResponse {
            action: self.state.clone().into(),
            lights: self.lights.clone(),
            name: self.name.clone(),
            rtype: HueV1GroupTypes::LightGroup,
            state: HueV1GroupState { all_on, any_on },
            sensors: vec![], // TODO: Add sensor ids when support is added
            lightlevel: None,
            presence: None,
            recycle: false,
        }
    }

    pub fn get_v1_state_simple(&self) -> HueV1GroupSimpleResponse {
        HueV1GroupSimpleResponse {
            action: self.state.clone().into(),
            lights: self.lights.clone(),
            name: self.name.clone(),
        }
    }
}
