use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};


pub type HueV1GroupMapResponse = BTreeMap<u8, HueV1GroupResponseWOState>;

#[derive(Serialize, Deserialize, Debug)]
pub struct HueV1GroupResponse {
    pub name: String,
    pub lights: Vec<u8>,
    #[serde(rename = "type")]
    pub rtype: String, // "LightGroup"
    pub action: Action,
    pub state: State,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HueV1GroupResponseWOState {
    pub action: Action,
    pub lights: Vec<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub rtype: String, // "Room"
    pub modelid: String,
    pub uniqueid: String,
    pub class: String
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub any_on: bool,
    pub all_on: bool,
}