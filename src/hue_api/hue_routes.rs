use actix_web::{error, get, post, web, HttpResponse, Responder};
use futures::StreamExt;
use log::info;
use serde::{Deserialize, Serialize};

use crate::util::mac_addr_to_bridge_id;
use crate::{
    bridge::config_get_mac_addr, hue_api::hue_config_controller::HueConfigControllerState,
};

#[derive(Serialize)]
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
            apiversion: "1.50.0".to_string(),
            bridgeid: mac_addr_to_bridge_id(&config_get_mac_addr()),
            datastoreversion: "103".to_string(),
            factorynew: false,
            mac: config_get_mac_addr(),
            modelid: "BSB002".to_string(),
            name: "Rustue".to_string(),
            replacesbridgeid: "".to_string(),
            starterkitid: "".to_string(),
            swversion: "1950207110".to_string(),
        }
    }
}

// //{"success":{"username": "` + username + `"}}

// #[macro_export]
// macro_rules! hue_success_json {
//     ($($key:expr => $value:expr),*) => {
//         json!([{ "success": { $($key: $value),* } }]).to_string()
//     };
// }

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserData {
    pub devicetype: String,
    pub generateclientkey: bool,
}

fn json_resp(body: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[post("")]
pub async fn route_config_post(
    mut payload: web::Payload,
    api_state: web::Data<HueConfigControllerState>,
) -> impl Responder {
    // For some odd reason actix cant deserialize the payload into a CreateUserData
    // so we have to do it manually.
    const MAX_SIZE: usize = 262_144; // 256 KB

    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // print
    println!("{}", String::from_utf8_lossy(&body));
    let obj = serde_json::from_slice::<CreateUserData>(&body).unwrap();

    if !api_state.get_controller().is_link_button_pressed() {
        // 101 Error - Link button not pressed
        // TODO: Define error codes with messages
        // TODO: Implement macro for error response
        info!("Link button not pressed");
        let resp = json!([{ "error": { "type": 101, "address": "/api/", "description": "link button not pressed" } }]).to_string();
        json_resp(resp);
    }

    let uuid = api_state.get_controller().add_user(&obj.devicetype);
    let resp = json!([{ "success": { "username": uuid, "clientkey": "321c0c2ebfa7361e55491095b2f5f9db" } }]).to_string();
    Ok(json_resp(resp))
}

#[get("/config")]
pub async fn route_config(api_state: web::Data<HueConfigControllerState>) -> impl Responder {
    let bridge_config = &api_state.get_controller().bridge_config;
    let bridgeid = &bridge_config.bridgeid;
    let mac = &bridge_config.mac;
    let response = HueConfigResponse {
        bridgeid: bridgeid.to_string(),
        mac: mac.to_string(),
        ..HueConfigResponse::default()
    };
    json_resp(json!(response).to_string())
}

// #[get("/nouser/config")]
// pub fn route_config_no_user(
//     api_state: &State<HueConfigControllerState>,
// ) -> content::RawJson<String> {
//     let bridge_config = &api_state.get_controller().bridge_config;

//     let bridgeid = &bridge_config.bridgeid;
//     let mac = &bridge_config.mac;
//     let response = HueConfigResponse {
//         bridgeid: bridgeid.to_string(),
//         mac: mac.to_string(),
//         ..HueConfigResponse::default()
//     };
//     content::RawJson(json!(response).to_string())
// }

// #[get("/<uid>/config")]
// pub fn route_config_with_uid(
//     uid: String,
//     api_state: &State<HueConfigControllerState>,
// ) -> content::RawJson<String> {
//     println!("uid: {}", uid);
//     let bridge_config = &api_state.get_controller().bridge_config;

//     let bridgeid = &bridge_config.bridgeid;
//     let mac = &bridge_config.mac;
//     let response = HueConfigResponse {
//         bridgeid: bridgeid.to_string(),
//         mac: mac.to_string(),
//         ..HueConfigResponse::default()
//     };
//     content::RawJson(json!(response).to_string())
// }
