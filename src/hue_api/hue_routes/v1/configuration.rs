use std::collections::HashMap;

use crate::hue_api::{
    hue_routes::{V1ApiUserGuard, SharedState},
    types::v1::{
        configuration::HueV1ConfigurationResponse, datastore::HueV1DatastoreResponse,
        light::HueV1LightMapResponse, responses::HueV1SmallConfigResponse, Swupdate,
    },
};
use actix_web::{error, get, post, put, web, HttpResponse, Responder};
use futures::StreamExt;
use log::debug;
use serde::Deserialize;

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
pub async fn create_user(mut payload: web::Payload, api_state: SharedState) -> impl Responder {
    let resp: String;
    if !api_state.get_controller_write().is_link_button_pressed() {
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

#[get("/{uid}")]
pub async fn get_full_datastore(_uid: V1ApiUserGuard, api_state: SharedState) -> impl Responder {
    let controller = &api_state.get_controller_read();
    web::Json(HueV1DatastoreResponse::build(
        &controller.bridge_config,
        HueV1LightMapResponse::build(&controller.light_devices),
        controller.group_map.get_v1(),
    ))
}

#[get("/{uid}/config")]
pub async fn get_configuration(_uid: V1ApiUserGuard, api_state: SharedState) -> impl Responder {
    let controller = &api_state.get_controller_read();
    web::Json(HueV1ConfigurationResponse::from(&controller.bridge_config))
}

#[get("/config")]
pub async fn get_config(api_state: SharedState) -> impl Responder {
    let bridge_config = &api_state.get_controller_read().bridge_config;
    web::Json(HueV1SmallConfigResponse::from(bridge_config))
}

#[derive(Deserialize, Debug)]
pub struct NewConfiguration {
    proxyport: Option<String>,
    name: Option<String>,
    swupdate: Option<Swupdate>,
    proxyaddress: Option<String>,
    linkbutton: Option<bool>,
    ipaddress: Option<String>,
    netmask: Option<String>,
    gateway: Option<String>,
    dhcp: Option<bool>,
    #[serde(rename = "UTC")]
    utc: Option<String>,
    timezone: Option<String>,
    touchlink: Option<bool>,
    zigbeechannel: Option<u8>,
}

#[put("/{uid}/config")]
pub async fn modify_configuration(
    _uid: V1ApiUserGuard,
    api_state: SharedState,
    params: web::Json<NewConfiguration>,
) -> impl Responder {
    // Sample response: [{"success":{"/config/name":"My bridge"}}]
    let _controller = &mut api_state.get_controller_write();
    let newconfg = params.into_inner();

    debug!("New configuration: {:?}", newconfg);

    let _changed_params: HashMap<String, String> = HashMap::new();

    // controller.bridge_config.name = newconfg.name.unwrap_or(controller.bridge_config.name);
    "TODO"
}
