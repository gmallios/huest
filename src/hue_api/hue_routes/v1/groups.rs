use actix_web::{delete, get, post, put, web, Responder};
use serde::Deserialize;

use crate::hue_api::hue_routes::{APIUserGuard, SharedState};

#[get("/{uid}/groups")]
pub async fn get_all_groups(_uid: APIUserGuard, _api_state: SharedState) -> impl Responder {
    // Sample Response:
    // {
    //     "1": {
    //         "name": "Group 1",
    //         "lights": [
    //             "1",
    //             "2"
    //         ],
    //         "type": "LightGroup",
    //         "action": {
    //             "on": true,
    //             "bri": 254,
    //             "hue": 10000,
    //             "sat": 254,
    //             "effect": "none",
    //             "xy": [
    //                 0.5,
    //                 0.5
    //             ],
    //             "ct": 250,
    //             "alert": "select",
    //             "colormode": "ct"
    //         }
    //     },
    //     "2": {
    //         "name": "Group 2",
    //         "lights": [
    //             "3",
    //             "4",
    //             "5"
    //         ],
    //         "type": "LightGroup",
    //         "action": {
    //             "on": true,
    //             "bri": 153,
    //             "hue": 4345,
    //             "sat": 254,
    //             "effect": "none",
    //             "xy": [
    //                 0.5,
    //                 0.5
    //             ],
    //             "ct": 250,
    //             "alert": "select",
    //             "colormode": "ct"
    //         }
    //     }
    // }
    "TODO"
}

#[derive(Deserialize)]
pub struct NewGroup {
    // Not final
    name: String,
    #[serde(rename = "type")]
    group_type: String,
    lights: Vec<String>,
}

#[post("/{uid}/groups")]
pub async fn create_new_group(
    _uid: APIUserGuard,
    _api_state: SharedState,
    _body: web::Json<NewGroup>,
) -> impl Responder {
    // Sample Response: [{"success":{"id":"1"}}]
    "TODO"
}

#[get("/{uid}/groups/{group_id}")]
pub async fn get_group(
    _uid: APIUserGuard,
    _group_id: web::Path<String>,
    _api_state: SharedState,
) -> impl Responder {
    // Sample Response
    // {
    //     "action": {
    //         "on": true,
    //         "hue": 0,
    //         "effect": "none",
    //         "bri": 100,
    //         "sat": 100,
    //         "ct": 500,
    //         "xy": [0.5, 0.5]
    //     },
    //     "lights": [
    //         "1",
    //         "2"
    //     ],
    //         "state":{"any_on":true, "all_on":true}   "type":"Room",   "class":"Bedroom",   "name":"Master bedroom", }
    "TODO"
}

#[derive(Deserialize)]
pub struct GroupAttributes {
    name: Option<String>,
    lights: Option<Vec<String>>, // Array of light ids
    class: Option<String>,       // Category
}

#[put("{uid}/groups/{group_id}")]
pub async fn set_group_attr(
    _uid: APIUserGuard,
    _group_id: web::Path<String>,
    _api_state: SharedState,
    _attr: web::Json<GroupAttributes>,
) -> impl Responder {
    // Sample Response:
    // [
    // {"success":{"/groups/1/lights":["1"]}},
    // {"success":{"/groups/1/name":"Bedroom"}}
    // ]
    "TODO"
}

#[derive(Deserialize)]
pub struct NewGroupState {
    on: Option<bool>,
    bri: Option<u8>,
    hue: Option<u16>,
    sat: Option<u8>,
    xy: Option<Vec<f32>>,
    ct: Option<u16>,
    alert: Option<String>,
    effect: Option<String>,
    transitiontime: Option<u16>,
    bri_inc: Option<i16>,     // -254 to 254
    sat_inc: Option<i16>,     // -254 to 254
    hue_inc: Option<i32>,     // -65534 to 65534
    ct_inc: Option<i32>,      // -65534 to 65534
    xy_inc: Option<Vec<f64>>, // Max [0.5, 0.5]
    scene: Option<String>,
}

#[put("{uid}/groups/{group_id}/action")]
pub async fn set_group_state(
    _uid: APIUserGuard,
    _api_state: SharedState,
    _group_id: web::Path<String>,
    _new_state: web::Json<NewGroupState>,
) -> impl Responder {
    // Sample Response:
    // [
    //     {"success":{ "address": "/groups/1/action/on", "value": true}},
    //     {"success":{ "address": "/groups/1/action/effect", "value":"colorloop"}},
    //     {"success":{ "address": "/groups/1/action/hue", "value":6000}}
    // ]
    "TODO"
}

#[delete("/{uid}/groups/{group_id}")]
pub async fn delete_group(
    _uid: APIUserGuard,
    _group_id: web::Path<String>,
    _api_state: SharedState,
) -> impl Responder {
    // Sample Response:
    // [{
    //     "success": "/groups/1 deleted."
    // }]
    "TODO"
}
