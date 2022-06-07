use crate::{bridge::config_get_mac_addr, hue_api::hue_config_controller::HueConfigControllerState};
use rocket::{response::content, Route, State};
use serde_json::json;

mod device_model;
pub(crate) mod hue_config_controller;
mod hue_config_model;

// All routes under /api
pub fn hue_routes() -> Vec<Route> {
    routes![route_config, route_config_no_user, route_config_with_uid]
}

#[get("/config")]
fn route_config(api_state: &State<HueConfigControllerState>) -> content::RawJson<&'static str> {
    println!(
        "{:?}",
        api_state
            .hue_config_controller
            .lock()
            .unwrap()
            .get_device_list()
            .get(&0)
    );
    content::RawJson("{ 'devicetype': 'Rustue' }")
}

#[get("/nouser/config", data="<devicetype>")]
fn route_config_no_user(devicetype: Option<String>) -> content::RawJson<String> {
    
    content::RawJson(
        json!({
            "name": "Rustue Emulated Bridge",
            "datastoreversion": "103",
            "swversion": "1948086000",
            "apiversion": "1.47.0",
            "mac": config_get_mac_addr(),
            "bridgeid": "",
            "factorynew": false,
            "replacesbridgeid": null,
            "modelid": "hueConfig.config.config.name",
            "starterkitid": ""
        })
        .to_string(),
    )
}

#[get("/<uid>/config")]
fn route_config_with_uid(uid: usize) -> content::RawJson<String> {
    println!("uid: {}", uid);
    content::RawJson(
        json!({
            "name": "Rustue Emulated Bridge",
            "datastoreversion": "103",
            "swversion": "1948086000",
            "apiversion": "1.47.0",
            "mac": config_get_mac_addr(),
            "bridgeid": "",
            "factorynew": false,
            "replacesbridgeid": null,
            "modelid": "hueConfig.config.config.name",
            "starterkitid": ""
        })
        .to_string(),
    )
}
