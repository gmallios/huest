use crate::util::mac_addr_to_bridge_id;
use crate::{
    bridge::config_get_mac_addr, hue_api::hue_config_controller::HueConfigControllerState,
};
use rocket::response::content::RawJson;
use rocket::serde::Serialize;
use rocket::{response::content, Route, State};

mod device_model;
pub(crate) mod hue_config_controller;
mod hue_config_model;

// All routes under /api
pub fn hue_routes() -> Vec<Route> {
    routes![
        create_user_route,
        route_config,
        route_config_no_user,
        route_config_with_uid
    ]
}

#[get("/", data = "<devicetype>")]
fn create_user_route(devicetype: Option<String>) -> content::RawJson<String> {
    println!("{:?}", devicetype);
    content::RawJson(json!({"success":{"username": "83b7780291a6ceffbe0bd049104df"}}).to_string())
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct HueConfigResponse {
    apiversion: String,
    bridgeid: String,
    datastoreversion: String,
    factorynew: bool,
    mac: String,
    modelid: String,
    name: String,
    replacesbridgeid: String,
    starterkitid: String,
    swversion: String,
}

impl Default for HueConfigResponse {
    fn default() -> Self {
        HueConfigResponse {
            apiversion: "1.47.0".to_string(),
            bridgeid: mac_addr_to_bridge_id(&config_get_mac_addr()),
            datastoreversion: "1".to_string(),
            factorynew: false,
            mac: config_get_mac_addr(),
            modelid: "BSB002".to_string(),
            name: "Rustue".to_string(),
            replacesbridgeid: "".to_string(),
            starterkitid: "".to_string(),
            swversion: "1948086000".to_string(),
        }
    }
}

#[get("/config")]
fn route_config(api_state: &State<HueConfigControllerState>) -> RawJson<String> {
    let bridge_config = api_state
        .hue_config_controller
        .lock()
        .unwrap()
        .bridge_config
        .clone();

    let bridgeid = bridge_config.bridgeid.clone();
    let mac = bridge_config.mac.clone();
    let response = HueConfigResponse {
        bridgeid: bridgeid,
        mac: mac,
        ..HueConfigResponse::default()
    };
    content::RawJson(json!(response).to_string())
}

#[get("/nouser/config", data = "<devicetype>")]
fn route_config_no_user(devicetype: Option<String>, api_state: &State<HueConfigControllerState>) -> content::RawJson<String> {
    println!("{:?}", devicetype);
    let bridge_config = api_state
        .hue_config_controller
        .lock()
        .unwrap()
        .bridge_config
        .clone();

    let bridgeid = bridge_config.bridgeid.clone();
    let mac = bridge_config.mac.clone();
    let response = HueConfigResponse {
        bridgeid: bridgeid,
        mac: mac,
        ..HueConfigResponse::default()
    };
    content::RawJson(json!(response).to_string())
}

#[get("/<uid>/config")]
fn route_config_with_uid(uid: String, api_state: &State<HueConfigControllerState>) -> content::RawJson<String> {
    println!("uid: {}", uid);
    let bridge_config = api_state
        .hue_config_controller
        .lock()
        .unwrap()
        .bridge_config
        .clone();

    let bridgeid = bridge_config.bridgeid.clone();
    let mac = bridge_config.mac.clone();
    let response = HueConfigResponse {
        bridgeid: bridgeid,
        mac: mac,
        ..HueConfigResponse::default()
    };
    content::RawJson(json!(response).to_string())
}
