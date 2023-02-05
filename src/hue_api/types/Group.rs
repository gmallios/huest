// use std::collections::HashMap;

// use serde::{Deserialize, Serialize};

// pub type HueGroupMap = HashMap<u8, HueGroupItem>;

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct HueGroupItem {
//     pub id: u8,
//     pub name: String,
//     pub lights: Vec<u8>,
//     #[serde(rename = "type")]
//     pub rtype: String, // https://developers.meethue.com/develop/hue-api/groupds-api/
//     pub action: Action,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Action {
//     pub on: bool,
//     pub bri: u8,
//     pub hue: u16,
//     pub sat: u8,
//     pub effect: Option<String>,
//     pub xy: Option<[f32; 2]>,
//     pub ct: Option<u16>,
//     pub alert: Option<String>,
//     pub colormode: Option<String>,
//     pub mode: Option<String>,
// }