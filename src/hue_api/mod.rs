use rocket::Route;

pub mod hue_device_model;
pub mod hue_config_controller;
pub mod hue_config_model;
pub mod hue_mdns;

pub use hue_config_controller as config_controller;
pub use hue_config_model as config_model;
pub use hue_device_model as device_model;
pub use hue_mdns as mdns;


mod hue_routes;
use hue_routes::*;



// TODO: Proper module split
// https://stackoverflow.com/questions/22596920/split-a-module-across-several-files

// All routes under /api
pub fn hue_routes() -> Vec<Route> {
    routes![
        route_config,
        route_config_no_user,
        route_config_with_uid,
        route_config_post,
        press_link_button,
        is_link_button_pressed
    ]
}

// Debug routes
#[get("/slink")]
fn press_link_button(api_state: &rocket::State<hue_config_controller::HueConfigControllerState>) -> String {
    api_state.get_controller().press_link_button();
    "Link button pressed".to_string()
}

#[get("/islink")]
fn is_link_button_pressed(api_state: &rocket::State<hue_config_controller::HueConfigControllerState>) -> String {
    println!("is_link_button_pressed {} ", api_state.get_controller().is_link_button_pressed());
    if api_state.get_controller().is_link_button_pressed() {
        "Link button pressed".to_string()
    } else {
        "Link button not pressed".to_string()
    }
}
