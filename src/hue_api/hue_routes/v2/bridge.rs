use crate::hue_api::{
    hue_routes::{HueApplicationKeyGuard, SharedState},
    types::v2::{responses::*, *},
};
use actix_web::{get, web, Responder, Result};

#[get("")]
pub async fn get_all(
    api_state: SharedState,
    guard: HueApplicationKeyGuard,
) -> Result<impl Responder> {
    let bridge_config = &api_state.get_controller_read().bridge_config;
    let mut data = vec![
        json!(Homekit::from(bridge_config)),
        json!(BridgeDevice::from(bridge_config)),
        json!(BridgeResource::from(bridge_config)),
        json!(Zigbee::from(bridge_config)),
        json!(Entertainment::from(bridge_config)),
        json!(GroupedLight::default()), /* INCOMPLETE - Should map every Group from config to a GroupedLight */
        json!(BridgeHome::from(bridge_config)),
        json!(GeofenceClient::from(&guard)),
    ];
    data.append(&mut behavior_scripts());

    Ok(web::Json(ResourceResponse {
        errors: vec![],
        data,
    }))
}

#[get("/bridge")]
pub async fn get_bridge(api_state: SharedState) -> Result<impl Responder> {
    let bridge_config = &api_state.get_controller_read().bridge_config;
    Ok(web::Json(ResourceResponse {
        data: vec![BridgeResource::from(bridge_config)],
        errors: vec![],
    }))
}
