use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap;



// Device.yaml type definitions

pub type DeviceMap = HashMap<u8, Device>;

#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct Device {
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

#[derive(Serialize, Deserialize,Clone,Debug)]
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

#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct Startup {
    #[serde(rename = "mode")]
    mode: String,

    #[serde(rename = "configured")]
    configured: bool,
}

#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct ProtocolCfg {
    #[serde(rename = "ip")]
    ip: String,

    #[serde(rename = "id")]
    id: Option<String>,

    #[serde(rename = "backlight")]
    backlight: Option<bool>,

    #[serde(rename = "model")]
    model: Option<String>,

    #[serde(rename = "ledCount")]
    led_count: Option<i64>,

    #[serde(rename = "mdns_name")]
    mdns_name: Option<String>,

    #[serde(rename = "mac")]
    mac: Option<String>,

    #[serde(rename = "segmentId")]
    segment_id: Option<i64>,
}

#[derive(Serialize, Deserialize,Clone,Debug)]
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
