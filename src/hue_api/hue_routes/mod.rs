use actix_web::{get, web};

use self::configuration::*;

use super::hue_config_controller::{self, HueConfigControllerState};

mod configuration;
mod lights;
mod groups;

pub type SharedController = web::Data<HueConfigControllerState>;
pub type UIDParam = web::Path<String>; 

// All routes under /api
pub fn get_hue_api_routes() -> actix_web::Scope {
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
