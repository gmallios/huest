use std::sync::{Mutex, Arc};

use hue_api::hue_config_controller::HueConfigController;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;

mod bridge;
mod hue_api;

// use crate::hue_api::hue_config_controller::HUE_CONFIG_CONTROLLER;

// fn main() {

//     // HttpServer::new(
//     //     || {
//     //         App::new()
//     //             .configure(api_config)
//     //             .wrap(middleware::Logger::default())
//     //             .wrap(middleware::NormalizePath::trim())
//     //     }, // No need to add / at the end of the URL
//     // )
//     // //.bind((bridge::BRIDGE_PARAMS.lock().unwrap().bind_ip.clone(), bridge::BRIDGE_PARAMS.lock().unwrap().port.clone()))?
//     // .bind(("0.0.0.0", 6565))?
//     // .run()
//     // .await

// }

pub struct ApiState {
    hue_config_controller: Arc<Mutex<HueConfigController>>,
}

#[launch]
fn rocket() -> _ {
    // Create HUE_CONFIG_CONTORLLER

    // lazy_static::initialize(&HUE_CONFIG_CONTROLLER);
    // println!(
    //     "{:?}",
    //     HUE_CONFIG_CONTROLLER
    //         .read()
    //         .unwrap()
    //         .get_device_list()
    //         .get(&0)
    // );

    let api_state = ApiState {
        hue_config_controller: Arc::new(Mutex::new(HueConfigController::new())),
    };

    rocket::build()
        .manage(api_state)
        .mount("/", routes![hello])
        .mount("/api", hue_api::hue_routes())
}

#[get("/")]
async fn hello() -> &'static str {
    "Hello world!"
}

// fn api_config(cfg: &mut web::ServiceConfig) {
//     cfg.service(hello)
//         .service(web::scope("/api").configure(hue_api::hue_services));
// }

// fn create_devices_config() -> std::io::Result<std::fs::File> {
//     use std::fs;

//     let device_config = fs::File::create("config/Devices.yaml").expect("Could not create Devices.yaml");
//     Ok(device_config)
// }

// fn create_config_dir() -> std::io::Result<()> {
//     use std::fs;

//     let config_dir = fs::create_dir("config");
//     Ok(())

// }
