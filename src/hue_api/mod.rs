use rocket::Route;

pub mod hue_device_model;
pub mod hue_config_controller;
pub mod hue_config_model;

pub use hue_config_controller as config_controller;
pub use hue_config_model as config_model;
pub use hue_device_model as device_model;


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
        press_link_button
    ]
}

// Debug routes
#[get("/slink")]
fn press_link_button(api_state: &rocket::State<hue_config_controller::HueConfigControllerState>) {
    api_state.get_controller().press_link_button();
}
