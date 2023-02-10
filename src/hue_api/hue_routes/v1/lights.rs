use actix_web::{delete, get, post, put, web, Responder};
use serde::Deserialize;

use crate::hue_api::{
    hue_routes::{SharedState, V1ApiUserGuard},
    types::v1::light::HueV1LightMapResponse,
};

use crate::hue_api::types::v1::light::HueV1NewLightState;

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
    let resp = HueV1LightMapResponse::build(&controller.light_instances.read().unwrap()).await;
    web::Json(resp)
}

#[get("/{uid}/lights/{lid}")]
pub async fn get_light(
    params: web::Path<(V1ApiUserGuard, u8)>,
    api_state: SharedState,
) -> impl Responder {
    let lid = params.1;
    let lights = &api_state.get_controller_read().light_instances;
    if let Some(light) = lights.read().unwrap().get(&lid) {
        return web::Json(json!(light.get_v1_state()));
    }

    web::Json(serde_json::Value::Null)
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

#[put("/{uid}/lights/{light_id}/state")]
pub async fn set_light_state(
    params: web::Path<(V1ApiUserGuard, u8)>,
    new_state: web::Json<HueV1NewLightState>,
    api_state: SharedState,
) -> impl Responder {
    let lid = params.1;
    let lights = &api_state.get_controller_read().light_instances;
    if let Some(light) = lights.read().unwrap().get(&lid) {
        light.set_v1_state(new_state.clone()).await;
        let mut res = vec![];
        if let Some(on) = new_state.on {
            res.push(json!({
                "success": {
                    "/lights/{lid}/state/on": on
                }
            }));
        }
        if let Some(bri) = new_state.bri {
            res.push(json!({
                "success": {
                    "/lights/{lid}/state/bri": bri
                }
            }));
        }
        if let Some(hue) = new_state.hue {
            res.push(json!({
                "success": {
                    "/lights/{lid}/state/hue": hue
                }
            }));
        }
        if let Some(sat) = new_state.sat {
            res.push(json!({
                "success": {
                    "/lights/{lid}/state/sat": sat
                }
            }));
        }
        if let Some(ct) = new_state.ct {
            res.push(json!({
                "success": {
                    "/lights/{lid}/state/ct": ct
                }
            }));
        }
        if let Some(xy) = new_state.xy {
            res.push(json!({
                "success": {
                    "/lights/{lid}/state/xy": xy
                }
            }));
        }
        if let Some(alert) = &new_state.alert {
            res.push(json!({
                "success": {
                    "/lights/{lid}/state/alert": alert
                }
            }));
        }
        if let Some(effect) = &new_state.effect {
            res.push(json!({
                "success": {
                    "/lights/{lid}/state/effect": effect
                }
            }));
        }
        return web::Json(json!(res));
    }

    web::Json(serde_json::Value::Null) /* TODO: Build correct response */
}

#[delete("/{uid}/lights/{light_id}")]
pub async fn delete_light(
    _uid: V1ApiUserGuard,
    _light_id: web::Path<String>,
    _api_state: SharedState,
) -> impl Responder {
    "TODO"
}
