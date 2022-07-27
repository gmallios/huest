use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

// Device.yaml type definitions
// Questonable name
// HueLightMap??
pub type HueDeviceMap = HashMap<u8, HueDeviceItem>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HueDeviceItem { 
    #[serde(rename = "id_v2")]
    pub id_v2: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "modelid")]
    pub modelid: String,
    #[serde(rename = "uniqueid")]
    pub uniqueid: String,
    #[serde(rename = "state")]
    pub state: State,
    #[serde(rename = "config")]
    pub config: Config,
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[serde(rename = "protocol_cfg")]
    pub protocol_cfg: ProtocolCfg,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(rename = "archetype")]
    archetype: String,
    #[serde(rename = "function")]
    function: String,
    #[serde(rename = "direction")]
    direction: String,
    #[serde(rename = "startup")]
    startup: Startup,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Startup {
    #[serde(rename = "mode")]
    mode: String,
    #[serde(rename = "configured")]
    configured: bool,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {
    #[serde(rename = "on")]
    on: bool,
    #[serde(rename = "bri")]
    bri: i64,
    #[serde(rename = "hue")]
    hue: i64,
    #[serde(rename = "sat")]
    sat: i64,
    #[serde(rename = "xy")]
    xy: Vec<f64>,
    #[serde(rename = "ct")]
    ct: i64,
    #[serde(rename = "alert")]
    alert: String,
    #[serde(rename = "mode")]
    mode: String,
    #[serde(rename = "effect")]
    effect: String,
    #[serde(rename = "colormode")]
    colormode: String,
    #[serde(rename = "reachable")]
    reachable: bool,
}

// Used for specifying protocol-specific config params
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ProtocolCfg {
    WLEDProtocolCfg
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WLEDProtocolCfg {
    #[serde(rename = "ip")]
    ip: String,
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "backlight")]
    backlight: bool,
    #[serde(rename = "model")]
    model: String,
    #[serde(rename = "ledCount")]
    led_count: i64,
    #[serde(rename = "mdns_name")]
    mdns_name: String,
    #[serde(rename = "mac")]
    mac: String,
    #[serde(rename = "segmentId")]
    segment_id: i64,
}
