use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::hue_api::types::internal::{InternalGroupState, GroupInstance};

#[derive(Serialize)]
pub struct HueV1GroupMapSimpleResponse(BTreeMap<u8, HueV1GroupSimpleResponse>);


impl HueV1GroupMapSimpleResponse {
    pub fn build(groups: &BTreeMap<u8, GroupInstance>) -> HueV1GroupMapSimpleResponse {
        let mut groups_v1 = HueV1GroupMapSimpleResponse(BTreeMap::new());
        for (id, group) in groups.iter() {
            /* NOTE: SimpleResponse is used in DatastoreResponse in which Group 0 is skipped */
            if id == &0 {
                continue;
            }
            groups_v1.0.insert(*id, group.get_v1_state_simple());
        }
        groups_v1
    }
}

#[derive(Serialize)]
pub struct HueV1GroupMapResponse(BTreeMap<u8, HueV1GroupResponse>);

impl HueV1GroupMapResponse {
    pub fn build(groups: &BTreeMap<u8, GroupInstance>) -> HueV1GroupMapResponse {
        let mut groups_v1 = HueV1GroupMapResponse(BTreeMap::new());
        for (id, group) in groups.iter() {
            groups_v1.0.insert(*id, group.get_v1_state());
        }
        groups_v1
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HueV1GroupResponse {
    pub name: String,
    #[serde(rename = "type")]
    pub rtype: HueV1GroupTypes,
    pub lights: Vec<u8>,
    pub sensors: Vec<u8>, /* IDs of sensors */
    pub action: HueV1GroupSimpleAction,
    pub state: HueV1GroupState,
    pub presence: Option<HueV1PresenceGroupResponse>, /* Only exists if we have a Presence Sensor */
    pub lightlevel: Option<HueV1LightLevelGroupResponse>, /* Only exists if we have a Light Level Sensor */
    pub recycle: bool, /* If true - The resource should be automatically deleted  */
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HueV1PresenceGroupResponse {}

#[derive(Serialize, Deserialize, Debug)]
pub struct HueV1LightLevelGroupResponse {}




#[derive(Serialize, Deserialize, Debug)]
pub enum HueV1GroupTypes {
    LightGroup,
    Room,
    Zone,
    Entertainment,
    Luminaire,
    LightSource
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HueV1GroupSimpleResponse {
    pub lights: Vec<u8>,
    pub name: String,
    pub action: HueV1GroupAction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HueV1GroupAction {
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

impl From<InternalGroupState> for HueV1GroupAction {
    fn from(state: InternalGroupState) -> Self {
        Self {
            on: state.on,
            bri: state.bri,
            hue: state.hue,
            sat: state.sat,
            effect: state.effect,
            xy: state.xy,
            ct: state.ct,
            alert: state.alert,
            colormode: state.colormode,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HueV1GroupSimpleAction {
    pub on: bool,
    pub hue: u16,
    pub effect: String,
    pub bri: u8,
    pub sat: u8,
    pub xy: (f32, f32),
    pub ct: u16,
}

impl From<InternalGroupState> for HueV1GroupSimpleAction {
    fn from(state: InternalGroupState) -> Self {
        Self {
            on: state.on,
            bri: state.bri,
            hue: state.hue,
            sat: state.sat,
            effect: state.effect,
            xy: state.xy,
            ct: state.ct,
        }
    }
}



#[derive(Serialize, Deserialize, Debug)]
pub struct HueV1GroupState {
    pub any_on: bool,
    pub all_on: bool,
}
