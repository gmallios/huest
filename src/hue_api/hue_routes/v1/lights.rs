use actix_web::{delete, get, post, put, web, Responder};
use serde::Deserialize;

use crate::hue_api::{
    hue_routes::{SharedState, V1ApiUserGuard},
    types::v1::light::HueV1LightMapResponse,
};

#[post("/{uid}/lights")]
pub async fn scan_for_new_lights(_uid: V1ApiUserGuard, _api_state: SharedState) -> impl Responder {
    let resp = json!([
        {
            "success": {
                "/lights": "Searching for new devices"
            }
        }
    ]);
    web::Json(resp)
}

#[get("/{uid}/lights/new")]
pub async fn get_new_lights(_uid: V1ApiUserGuard, _api_state: SharedState) -> impl Responder {
    // Sample Response:
    // {
    //     "7": {"name": "Hue Lamp 7"},
    //     "8": {"name": "Hue Lamp 8"},
    //     "lastscan": "2012-10-29T12:00:00"
    // }
    "TODO"
}

#[get("/{uid}/lights")]
pub async fn get_all_lights(_uid: V1ApiUserGuard, api_state: SharedState) -> impl Responder {
    let controller = api_state.get_controller_read();
    web::Json(HueV1LightMapResponse::build(&controller.light_devices))
}

#[get("/{uid}/lights/{light_id}")]
pub async fn get_light(
    _uid: V1ApiUserGuard,
    light_id: web::Path<u8>,
    api_state: SharedState,
) -> impl Responder {
    let lights = &api_state.get_controller_read().light_devices;
    lights.get(&light_id).map_or_else(
        || web::Json(json!({"error": {"type": 3, "address": format!("/lights/{}", light_id), "description": "resource, /lights/1, not available"}})),
        |light| web::Json(json!(light.get_v1_state())),
    )
}

#[derive(Deserialize)]
pub struct RenameReq {
    pub name: String,
}

#[put("/{uid}/lights/{light_id}")]
pub async fn rename_light(
    _uid: V1ApiUserGuard,
    _light_id: web::Path<String>,
    _body: web::Json<RenameReq>,
    _api_state: SharedState,
) -> impl Responder {
    // Sample Response: [{"success":{"/lights/1/name":"Bedroom Light"}}]
    "TODO"
}

#[derive(Deserialize)]
pub struct NewLightState {
    on: Option<String>,
    bri: Option<u8>,
    hue: Option<u16>,
    sat: Option<u8>,
    xy: Option<Vec<f64>>,
    ct: Option<u16>,
    alert: Option<String>,
    effect: Option<String>,
    transitiontime: Option<u16>,
    bri_inc: Option<i16>,     // -254 to 254
    sat_inc: Option<i16>,     // -254 to 254
    hue_inc: Option<i32>,     // -65534 to 65534
    ct_inc: Option<i32>,      // -65534 to 65534
    xy_inc: Option<Vec<f64>>, // Max [0.5, 0.5]
}

#[put("/{uid}/lights/{light_id}/state")]
pub async fn set_light_state(
    _uid: V1ApiUserGuard,
    _light_id: web::Path<String>,
    _body: web::Json<NewLightState>,
    _api_state: SharedState,
) -> impl Responder {
    // Sample Response:
    //     [
    //     {"success":{"/lights/1/state/bri":200}},
    //     {"success":{"/lights/1/state/on":true}},
    //     {"success":{"/lights/1/state/hue":50000}}
    // ]
    "TODO"
}

#[delete("/{uid}/lights/{light_id}")]
pub async fn delete_light(
    _uid: V1ApiUserGuard,
    _light_id: web::Path<String>,
    _api_state: SharedState,
) -> impl Responder {
    "TODO"
}
