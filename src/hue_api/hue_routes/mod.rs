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
    // TODO: Refactor to use multiple scopes 
    web::scope("/api")
        .service(press_link_button) // Debug route
        .service(is_link_button_pressed) // Debug route
        .service(save_config) // Debug route
        // Configuration routes
        .service(get_config)
        .service(create_user)
        .service(get_configuration)
        .service(get_full_datastore)
        .service(modify_configuration)
        
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
