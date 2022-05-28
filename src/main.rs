use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware};

#[macro_use]
extern crate lazy_static;
extern crate serde_json;

mod devices;
mod hue_api;
mod bridge;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // use std::fs;

    // let file = fs::read_to_string("config/Devices.yaml").unwrap();
    // let a: devices::DeviceList = serde_yaml::from_str(&file).unwrap();

    //bridge::BRIDGE_PARAMS.lock().unwrap().mac_address = "".to_string();

    HttpServer::new(|| App::new()
                        .configure(api_config)
                        .wrap(middleware::Logger::default())
                        .wrap(middleware::NormalizePath::trim()) // No need to add / at the end of the URL
                    )
        //.bind((bridge::BRIDGE_PARAMS.lock().unwrap().bind_ip.clone(), bridge::BRIDGE_PARAMS.lock().unwrap().port.clone()))?
        .bind(("0.0.0.0", 6565))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
    .service(web::scope("/api").configure(hue_api::hue_services));
}

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
