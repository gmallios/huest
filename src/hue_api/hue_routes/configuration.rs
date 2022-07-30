use crate::hue_api::hue_routes::{SharedController, UIDParam};
use crate::hue_api::hue_types::Responses::*;
use crate::hue_api::{
    hue_config_controller::HueConfigControllerState, hue_types::Responses::HueConfigurationResponse,
};

use actix_web::{error, get, post, web, HttpResponse, Responder};
use futures::StreamExt;
use log::{debug};
use serde::{Deserialize};

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
    pub generateclientkey: Option<bool>,
}

fn json_resp(body: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[post("")]
pub async fn route_config_post(
    mut payload: web::Payload,
    api_state: SharedController,
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


        let obj = serde_json::from_slice::<CreateUserData>(&body).unwrap();
        let (uid, clientkey) = api_state
            .get_controller_write()
            .add_user(&obj.devicetype, &obj.generateclientkey);
        match clientkey {
            Some(key) => {
                resp = json!([{ "success": { "username": uid, "clientkey": key } }]).to_string();
            }
            None => {
                resp = json!([{ "success": { "username": uid } }]).to_string();
            }
        }
    }
    Ok(json_resp(resp))
}

#[get("/config")]
pub async fn route_config(api_state: SharedController) -> impl Responder {
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

#[get("/{uid}")]
pub async fn route_uid(
    uid: UIDParam,
    api_state: SharedController,
) -> impl Responder {
    let controller = &api_state.get_controller_read();
    //TODO: Reject if user auth fails
    println!("user_exists: {}", &controller.user_exists(&uid));

    let resp = crate::hue_api::hue_types::Responses::DatastoreResponse::from_bridge_config(
        controller.bridge_config.clone(),
        Some(controller.device_map.clone()),
        Some(controller.group_map.clone()),
    );
    json_resp(resp)
}

#[get("/{uid}/config")]
pub async fn route_config_with_uid(
    uid: UIDParam,
    api_state: SharedController,
) -> impl Responder {
    let controller = &api_state.get_controller_read();
    //TODO: Reject if user auth fails
    println!("user_exists: {}", &controller.user_exists(&uid));

    let resp = HueConfigurationResponse::from_bridge_config(controller.bridge_config.clone(), None, None);
    json_resp(resp)
}
