use actix_web::{delete, get, post, put, web, Responder};
use serde::Deserialize;

use crate::hue_api::hue_routes::{V1ApiUserGuard, SharedState};

#[post("/{uid}/lights")]
pub async fn search_for_new_lights(_uid: V1ApiUserGuard, _api_state: SharedState) -> impl Responder {
    // Sample Response: [ { "success": { "/lights": "Searching for new devices" }}]
    "TODO"
}

#[get("/{uid}/lights")]
pub async fn get_all_lights(_uid: V1ApiUserGuard, _api_state: SharedState) -> impl Responder {
    // Sample Response:
    // {
    //     "1": {
    //             "state": {
    //                 "on": false,
    //                 "bri": 1,
    //                 "hue": 33761,
    //                 "sat": 254,
    //                 "effect": "none",
    //                 "xy": [
    //                     0.3171,
    //                     0.3366
    //                 ],
    //                 "ct": 159,
    //                 "alert": "none",
    //                 "colormode": "xy",
    //                 "mode": "homeautomation",
    //                 "reachable": true
    //             },
    //             "swupdate": {
    //                 "state": "noupdates",
    //                 "lastinstall": "2018-01-02T19:24:20"
    //             },
    //             "type": "Extended color light",
    //             "name": "Hue color lamp 7",
    //             "modelid": "LCT007",
    //             "manufacturername": "Philips",
    //             "productname": "Hue color lamp",
    //             "capabilities": {
    //                 "certified": true,
    //                 "control": {
    //                     "mindimlevel": 5000,
    //                     "maxlumen": 600,
    //                     "colorgamuttype": "B",
    //                     "colorgamut": [
    //                         [
    //                             0.675,
    //                             0.322
    //                         ],
    //                         [
    //                             0.409,
    //                             0.518
    //                         ],
    //                         [
    //                             0.167,
    //                             0.04
    //                         ]
    //                     ],
    //                     "ct": {
    //                         "min": 153,
    //                         "max": 500
    //                     }
    //                 },
    //                 "streaming": {
    //                     "renderer": true,
    //                     "proxy": false
    //                 }
    //             },
    //             "config": {
    //                 "archetype": "sultanbulb",
    //                 "function": "mixed",
    //                 "direction": "omnidirectional"
    //             },
    //             "uniqueid": "00:17:88:01:00:bd:c7:b9-0b",
    //             "swversion": "5.105.0.21169"
    //         }
    //     }
    "TODO"
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

#[get("/{uid}/lights/{light_id}")]
pub async fn get_light(
    _uid: V1ApiUserGuard,
    _light_id: web::Path<String>,
    _api_state: SharedState,
) -> impl Responder {
    // Sample Response
    // {
    //     "state": {
    //         "hue": 50000,
    //         "on": true,
    //         "effect": "none",
    //         "alert": "none",
    //         "bri": 200,
    //         "sat": 200,
    //         "ct": 500,
    //         "xy": [0.5, 0.5],
    //         "reachable": true,
    //         "colormode": "hs"
    //     },
    //     "type": "Living Colors",
    //     "name": "LC 1",
    //     "modelid": "LC0015",
    //     "swversion": "1.0.3"
    // }
    "TODO"
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
