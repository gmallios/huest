use actix_web::{get, web};

pub mod hue_config_controller;
pub mod hue_mdns;
pub mod hue_ssdp;
pub mod hue_util;

pub use hue_config_controller as config_controller;
pub use hue_mdns as mdns;
pub use hue_ssdp as ssdp;
pub use hue_util as util;

use self::hue_routes::Configuration::*;

mod devices;
mod hue_routes;
mod hue_types;


// TODO: Proper module split
// https://stackoverflow.com/questions/22596920/split-a-module-across-several-files

// All routes under /api
pub fn hue_routes() -> actix_web::Scope {
    web::scope("/api")
        .service(press_link_button) // Debug route
        .service(is_link_button_pressed) // Debug route
        .service(save_config) // Debug route
        .service(route_config)
        .service(route_config_post)
        .service(route_config_with_uid)
        .service(route_uid)
        
}

#[get("/save")]
async fn save_config(
    api_state: web::Data<hue_config_controller::HueConfigControllerState>,
) -> &'static str {
    api_state.get_controller_read().save();
    "OK"
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
