use actix_web::{
    get,
    middleware::{self, Logger},
    web, App, HttpRequest, HttpResponse, HttpServer,
};
use bridge::config_get_mac_addr;
use log::{info, error};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use ssdp::start_ssdp_broadcast;
use std::{
    default, fs,
    net::Ipv4Addr,
    process::Command,
    sync::{Arc, Mutex},
    thread,
};

use hue_api::hue_mdns::start_hue_mdns;
use hue_api::{
    hue_config_controller::{HueConfigController, HueConfigControllerState},
    hue_routes,
};

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;

#[cfg(target_os = "linux")]
static OPENSSL_PATH: &str = "/usr/bin/openssl";
#[cfg(target_os = "macos")]
static OPENSSL_PATH: &str = "/opt/homebrew/opt/openssl/bin/openssl";

mod bridge;
mod hue_api;
mod ssdp;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    info!("Starting Hue Bridge...");
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

    

    // Generate SSL Certificates
    match gen_ssl_cert() {
        Ok(_) => {
            info!("SSL certificates generated!");
        }
        Err(_) => {
            error!("Failed to generate SSL certificates!");
            std::process::exit(1);
        }
    }

    thread::spawn(|| start_ssdp_broadcast());
    thread::spawn(|| start_hue_mdns());

    let api_state = web::Data::new(HueConfigControllerState {
        hue_config_controller: Arc::new(Mutex::new(HueConfigController::new())),
    });

    // Debug thread 
    // let state = api_state.clone();
    // thread::spawn(move || {
    //     loop {
    //         info!(
    //                 "linkbutton {}",
    //                 state.get_controller().is_link_button_pressed()
    //             );
    //             thread::sleep(std::time::Duration::from_secs(1));
    //     }
    // });

    let mut openssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    openssl_builder.set_private_key_file("./ssl/private.pem", SslFiletype::PEM);
    openssl_builder
        .set_certificate_chain_file("./ssl/cert.pem")
        .unwrap();

    let ssl = true;

    

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .app_data(api_state.clone())
            .service(description_xml)
            .service(hue_routes())
            .wrap(Logger::default())
    })
    .bind_openssl("0.0.0.0:443", openssl_builder)?
    .bind("0.0.0.0:80")?
    .run()
    .await
}

#[get("/")]
async fn hello() -> &'static str {
    "Hello world!"
}

#[get("/description.xml")]
async fn description_xml() -> impl actix_web::Responder {
    let xml_file = fs::read_to_string("./static/description.xml").unwrap();
    HttpResponse::Ok()
        .content_type("application/xml")
        .body(xml_file)
}

fn gen_ssl_cert() -> Result<std::process::Output, std::io::Error> {
    use std::process::Command;

    let mac_addr = config_get_mac_addr().replace(":", "");
    let serial = format!(
        "{}fffe{}",
        mac_addr[..6].to_string(),
        mac_addr[6..].to_string()
    );
    let decimal_serial = format!("{}", u64::from_str_radix(&serial, 16).unwrap());
    let cmd = format!("{} req -new -days 3650 -config ssl/openssl.conf  -nodes -x509 -newkey ec -pkeyopt ec_paramgen_curve:P-256 -pkeyopt ec_param_enc:named_curve   -subj \"/C=NL/O=Philips Hue/CN={}\" -keyout ssl/private.pem -out ssl/cert.pem -set_serial {}",OPENSSL_PATH,serial,decimal_serial);
    Command::new("/bin/sh")
        .arg("-c")
        .arg(cmd)
        .output()
}
