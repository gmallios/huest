use std::collections::HashMap;

use serde::{Serialize, Deserialize};

pub type HueGroupMap = HashMap<u8, HueGroupItem>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HueGroupItem {
    pub id: u8,
    pub name: String,
    pub lights: Vec<u8>,
    #[serde(rename = "type")]
    pub grouptype: String,  // https://developers.meethue.com/develop/hue-api/groupds-api/
    pub action: Action
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    pub on: bool,
    pub bri: i64,
    pub hue: i64,
    pub sat: i64,
    pub effect: String,
    pub xy: Vec<f64>,
    pub ct: i64,
    pub alert: String,
    pub colormode: String,
}