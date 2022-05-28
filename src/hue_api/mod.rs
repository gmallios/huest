use crate::bridge::config_get_mac_addr;
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;


// All routes here as children of /api
pub fn hue_services(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
        .service(route_config)
        .service(route_config_no_user);
}

#[get("")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/config")]
async fn route_config() -> impl Responder {
    HttpResponse::Ok().body(json!({ "devicetype": "Rustue" }).to_string())
}

#[get("/nouser/config")]
async fn route_config_no_user() -> impl Responder {
    HttpResponse::Ok().body(
        json!({
            "name": "Rustue Emulated Bridge",
            "datastoreversion": "103",
            "swversion": "1948086000",
            "apiversion": "1.47.0",
            "mac": config_get_mac_addr(),
            "bridgeid": "",
            "factorynew": false,
            "replacesbridgeid": null,
            "modelid": "hueConfig.config.config.name",
            "starterkitid": ""
        })
        .to_string(),
    )
}
