use actix_web::{error, get, post, web, HttpResponse, Responder};
use futures::StreamExt;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use crate::hue_api::hue_types::Responses::*;
use crate::util::mac_addr_to_bridge_id;
use crate::{
    bridge::config_get_mac_addr, hue_api::hue_config_controller::HueConfigControllerState,
};


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
    let resp: String;
    if api_state.get_controller_read().is_link_button_pressed() == false {
        // 101 Error - Link button not pressed
        // TODO: Define error codes with messages
        // TODO: Implement macro for error response
        debug!("Attempted new user creation - Link button not pressed");
        resp = json!([{ "error": { "type": 101, "address": "/api/", "description": "link button not pressed" } }]).to_string();
    } else {
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

        debug!("{}", String::from_utf8_lossy(&body));
        let obj = serde_json::from_slice::<CreateUserData>(&body).unwrap();
        let uuid = api_state.get_controller_write().add_user(&obj.devicetype);
        resp = json!([{ "success": { "username": uuid, "clientkey": "321c0c2ebfa7361e55491095b2f5f9db" } }]).to_string();
    }
    Ok(json_resp(resp))
}

#[get("/config")]
pub async fn route_config(api_state: web::Data<HueConfigControllerState>) -> impl Responder {
    let bridge_config = &api_state.get_controller_read().bridge_config;
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

#[get("/{uid}/config")]
pub async fn route_config_with_uid(
    uid: web::Path<String>,
    api_state: web::Data<HueConfigControllerState>,
) -> impl Responder {
    println!("uid: {}", uid);
    let bridge_config = &api_state.get_controller_read().bridge_config;
    let resp = crate::hue_api::hue_types::Responses::DatastoreResponse::from_bridge_config(&DatastoreResponse::default(), bridge_config.clone());
    json_resp(resp)
}
