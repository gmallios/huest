use actix_web::{get, web};

pub mod hue_config_controller;
pub mod hue_mdns;

pub use hue_config_controller as config_controller;
pub use hue_mdns as mdns;

mod devices;
mod hue_routes;
mod hue_types;
mod hue_util;

use hue_routes::*;

// TODO: Proper module split
// https://stackoverflow.com/questions/22596920/split-a-module-across-several-files

// All routes under /api
pub fn hue_routes() -> actix_web::Scope {
    web::scope("/api")
        .service(route_config)
        .service(route_config_post)
        .service(route_config_with_uid)
        .service(press_link_button) // Debug routes
        .service(is_link_button_pressed) // Debug routes
}

#[get("/slink")]
async fn press_link_button(
    api_state: web::Data<hue_config_controller::HueConfigControllerState>,
) -> &'static str {
    api_state.get_controller_write().press_link_button();
    "Link button pressed"
}

#[get("/islink")]
async fn is_link_button_pressed(
    api_state: web::Data<hue_config_controller::HueConfigControllerState>,
) -> String {
    println!(
        "is_link_button_pressed {} ",
        api_state.get_controller_read().is_link_button_pressed()
    );
    if api_state.get_controller_read().is_link_button_pressed() {
        "Link button pressed".to_string()
    } else {
        "Link button not pressed".to_string()
    }
}
