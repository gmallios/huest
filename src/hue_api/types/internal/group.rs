use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::hue_api::types::v1::group::{
    HueV1GroupMapResponse, HueV1GroupResponse, HueV1GroupResponseWOState,
};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InternalGroupMap(pub BTreeMap<u8, InternalGroup>);

impl InternalGroupMap {
    pub fn get_v1(&self) -> HueV1GroupMapResponse {
        let mut groups = HueV1GroupMapResponse::new();
        for (id, group) in self.0.iter() {
            groups.insert(*id, group.get_v1_wo_state());
        }
        groups
    }
}

// Used to represent the internal state of a group which can be use to response to both V1 and V2 API
//
// InternalGroup -> impls GroupInstance -> fn v1_resp - Group
//                                      -> fn v2_resp - Room/Zone/Grouped Light
#[derive(Serialize, Deserialize, Debug)]
pub struct InternalGroup {
    name: String,
    id: String,
    id_v2: String,
    lights: Vec<String>, /* v2 IDs of lights */
    action: Action,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Action {
    pub on: bool,
    pub bri: u8,
    pub hue: u16,
    pub sat: u8,
    pub effect: Option<String>,
    pub xy: Option<[f32; 2]>,
    pub ct: Option<u16>,
    pub alert: Option<String>,
    pub colormode: Option<String>,
    pub mode: Option<String>,
}

impl InternalGroup {
    fn new(_group: &InternalGroup) -> Self {
        unimplemented!()
    }

    fn get_v1(&self) -> HueV1GroupResponse {
        unimplemented!()
    }
    fn get_v1_wo_state(&self) -> HueV1GroupResponseWOState {
        HueV1GroupResponseWOState {
            action: self.action.clone(),
            lights: self.lights.clone(),
            name: self.name.clone(),
            rtype: "Room".to_string(),
            modelid: "Group".to_string(),
            uniqueid: self.id.clone(),
            class: "Room".to_string(),
        }
    }
    fn get_v2_room(&self) {
        unimplemented!()
    }
    fn get_v2_zone(&self) {
        unimplemented!()
    }
    fn get_v2_grouped_light(&self) {
        unimplemented!()
    }
}

trait GroupInstance {
    fn new(group: &InternalGroup) -> Self
    where
        Self: Sized;
    fn get_v1(&self) -> HueV1GroupResponse;
    fn get_v1_wo_state(&self) -> HueV1GroupResponseWOState;
    fn get_v2_room(&self);
    fn get_v2_zone(&self);
    fn get_v2_grouped_light(&self);
}
