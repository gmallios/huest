use serde::{Deserialize, Serialize};

use crate::hue_api::{types::v1::light, util::rgb_to_xy};

#[derive(Serialize, Deserialize, Debug)]
pub struct WLEDInfoResponse {
    pub ver: String,
    pub vid: i64,
    pub leds: Leds,
    pub str: bool,
    pub name: String,
    pub udpport: i64,
    pub live: bool,
    pub lm: String,
    pub lip: String,
    pub ws: i64,
    pub fxcount: i64,
    pub palcount: i64,
    pub wifi: Wifi,
    pub fs: Fs,
    pub ndc: i64,
    pub arch: String,
    pub core: String,
    pub lwip: i64,
    pub freeheap: i64,
    pub uptime: i64,
    pub opt: i64,
    pub brand: String,
    pub product: String,
    pub mac: String,
    pub ip: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Leds {
    pub count: i64,
    pub pwr: i64,
    pub fps: i64,
    pub maxpwr: i64,
    pub maxseg: i64,
    pub seglc: Vec<i64>,
    pub lc: i64,
    pub rgbw: bool,
    pub wv: i64,
    pub cct: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wifi {
    pub bssid: String,
    pub rssi: i64,
    pub signal: i64,
    pub channel: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fs {
    pub u: i64,
    pub t: i64,
    pub pmt: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WLEDStateResponse {
    pub on: bool,
    pub bri: i64,
    pub transition: i64,
    pub ps: i64,
    pub pl: i64,
    pub nl: Nl,
    pub udpn: Udpn,
    pub lor: i64,
    pub mainseg: i64,
    pub seg: Vec<Seg>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Nl {
    on: bool,
    dur: i64,
    mode: i64,
    tbri: i64,
    rem: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Seg {
    pub id: u16,
    pub start: i64,
    pub stop: i64,
    pub len: i64,
    pub grp: i64,
    pub spc: i64,
    pub of: i64,
    pub on: bool,
    pub frz: bool,
    pub bri: i64,
    pub cct: i64,
    pub col: Vec<Vec<u8>>,
    pub fx: i64,
    pub sx: i64,
    pub ix: i64,
    pub pal: i64,
    pub sel: bool,
    pub rev: bool,
    pub mi: bool,
}

impl From<(WLEDStateResponse, Seg)> for light::State {
    fn from(value: (WLEDStateResponse, Seg)) -> Self {
        light::State {
            on: value.0.on,
            bri: value.0.bri,
            hue: 0,
            sat: 0,
            xy: rgb_to_xy(value.1.col[0][0], value.1.col[0][1], value.1.col[0][2]),
            ct: 0,
            alert: "none".into(),
            colormode: "xy".into(),
            mode: "homeautomation".into(),
            reachable: true,
            effect: "none".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Udpn {
    send: bool,
    recv: bool,
}
