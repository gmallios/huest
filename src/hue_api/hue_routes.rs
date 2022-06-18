use crate::util::mac_addr_to_bridge_id;
use crate::{
    bridge::config_get_mac_addr, hue_api::hue_config_controller::HueConfigControllerState,
};
use rocket::response::content::RawJson;
use rocket::http::uri::Origin;
use rocket::{serde::{Deserialize, Serialize, json::Json}, response::content, Route, State};

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
#[derive(Deserialize, Debug)]
pub struct CreateUserData {
    devicetype: String,
    generateclientkey: Option<bool>,
}

#[post("/", data = "<data>")]
pub fn route_config_post(origin: &Origin, data: Json<CreateUserData>, api_state: &State<HueConfigControllerState>) -> content::RawJson<String> {
    if !api_state.get_controller().is_link_button_pressed() {
        // 101 Error - Link button not pressed
        // TODO: Define error codes with messages
        // TODO: Implement macro for error response
        content::RawJson(json!({ "error": { "type": 101, "address": origin, "description": "link button not pressed" } }).to_string());
    }
    let uuid = api_state.get_controller().add_user(&data.devicetype);
    println!("devicetype: {}, generateclientkey: {}", data.devicetype, data.generateclientkey.unwrap());
    content::RawJson(json!(HueConfigResponse::default()).to_string())
}

#[get("/config")]
pub fn route_config(api_state: &State<HueConfigControllerState>) -> content::RawJson<String> {
    let bridge_config = api_state.get_bridge_config();
    let bridgeid = bridge_config.bridgeid.clone();
    let mac = bridge_config.mac.clone();
    let response = HueConfigResponse {
        bridgeid: bridgeid,
        mac: mac,
        ..HueConfigResponse::default()
    };
    content::RawJson(json!(response).to_string())
}

#[get("/nouser/config")]
pub fn route_config_no_user(api_state: &State<HueConfigControllerState>) -> content::RawJson<String> {
    let bridge_config = api_state.get_bridge_config();

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
pub fn route_config_with_uid(uid: String, api_state: &State<HueConfigControllerState>) -> content::RawJson<String> {
    println!("uid: {}", uid);
    let bridge_config = api_state.get_bridge_config();

    let bridgeid = bridge_config.bridgeid.clone();
    let mac = bridge_config.mac.clone();
    let response = HueConfigResponse {
        bridgeid: bridgeid,
        mac: mac,
        ..HueConfigResponse::default()
    };
    content::RawJson(json!(response).to_string())
}
