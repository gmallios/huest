use actix_web::{
    get,
    middleware::{self, Logger},
    web, App, HttpResponse, HttpServer,
};
use bridge::get_mac_addr;
use log::{error, info, warn, debug};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
//use rustls::{ServerConfig, Certificate, PrivateKey};
//use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{
    fs::{self},
    process::Command,
    sync::{Arc, RwLock},
    thread,
};

use hue_api::{
    bridge_config_controller::HueConfigController, hue_mdns::start_hue_mdns,
    hue_routes::hue_v1_routes, hue_ssdp::start_ssdp_broadcast,
};

use crate::hue_api::hue_routes::{hue_v2_clipstream, hue_v2_routes};

#[macro_use]
extern crate serde_json;

// This openssl path is used for creating the TLS certificate.
// TODO: Find a better way to do this in order to support other Platforms which do not have openssl.
//       rcgen crate is a good candidate.
// Generaly we prefer pure rust implementations of various functions in order to avoid the need for external dependencies.
#[cfg(target_os = "linux")]
static OPENSSL_PATH: &str = "/usr/bin/openssl";
#[cfg(target_os = "macos")]
static OPENSSL_PATH: &str = "/opt/homebrew/opt/openssl/bin/openssl";
#[cfg(target_os = "windows")]
static OPENSSL_PATH: &str = "openssl.exe";

mod bridge;
mod hue_api;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = env_logger::Builder::from_default_env();
    builder
        .filter(None, log::LevelFilter::Debug)
        .filter(Some("libmdns"), log::LevelFilter::Off)
        .filter(Some("h2"), log::LevelFilter::Off)
        .filter(Some("scraper"), log::LevelFilter::Off)
        .filter(Some("selectors"), log::LevelFilter::Off)
        .filter(Some("html5ever"), log::LevelFilter::Off)
        .filter(Some("rustls"), log::LevelFilter::Off)
        .filter(Some("hyper"), log::LevelFilter::Off)
        .filter(Some("reqwest"), log::LevelFilter::Off)
        .init();

    info!("Starting Hue Bridge...");

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

    thread::spawn(start_ssdp_broadcast);
    thread::spawn(start_hue_mdns);

    let api_state = web::Data::new(HueAppState {
        hue_config_controller: Arc::new(std::sync::RwLock::new(HueConfigController::new().await)),
        device_req_client: reqwest::Client::new(),
    });

    if let Some(ver) = hue_api::hue_util::get_latest_swversion().await {
        api_state
            .hue_config_controller
            .write()
            .unwrap()
            .update_swversion(&ver);
        log::debug!("got latest swversion: {}", ver);
    }

    let state = api_state.clone();
    // Sync Lights? - Fetch state periodcally
    actix_rt::spawn(async move {
        loop {
            // TODO: Make this configurable
            // TODO: Check if a user is connected to the bridge before refetching the state
            let config_controller = state.hue_config_controller.try_read();
            match config_controller {
                Ok(config_controller) => {
                    match config_controller.light_instances.try_read() {
                        Ok(light_instances) => {
                            for (id, device) in light_instances.iter() {
                                info!("Refetching state for device with id_v1: {} and name: {}", id, device.get_name());
                                device.refetch_state().await;
                            }
                            drop(light_instances);
                        }
                        Err(_) => {
                            warn!("Failed to get read lock on device map while refetching state!");
                        }
                    }
                }
                Err(_) => {
                    warn!("Failed to get read lock on device map while refetching state!");
                }
            }
            actix_rt::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    let mut openssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    openssl_builder.set_private_key_file("./ssl/private.pem", SslFiletype::PEM)?;
    openssl_builder
        .set_certificate_chain_file("./ssl/cert.pem")
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .app_data(api_state.clone())
            .service(description_xml)
            .service(hue_v1_routes())
            .service(hue_v2_routes())
            .service(hue_v2_clipstream())
            .wrap(Logger::default())
    })
    //.bind_rustls("0.0.0.0:443", load_rustls_config())?
    .bind_openssl("0.0.0.0:443", openssl_builder)? // TODO: Use the arg ip
    .bind("0.0.0.0:80")? // TODO: Use the arg ip
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
    let mac_addr = get_mac_addr().replace(':', "");
    let serial = format!("{}fffe{}", &mac_addr[..6], &mac_addr[6..]);
    let decimal_serial = format!("{}", u64::from_str_radix(&serial, 16).unwrap());
    let cmd = format!("{} req -new -days 3650 -config ssl/openssl.conf  -nodes -x509 -newkey ec -pkeyopt ec_paramgen_curve:P-256 -pkeyopt ec_param_enc:named_curve   -subj \"/C=NL/O=Philips Hue/CN={}\" -keyout ssl/private.pem -out ssl/cert.pem -set_serial {}",OPENSSL_PATH,serial,decimal_serial);

    let output = if cfg!(target_os = "windows") {
        warn!("Wonky ssl cert generation on windows, run: {}", &cmd);
        Command::new("cmd").args(["/C", &cmd]).output()
    } else {
        Command::new("sh").arg("-c").arg(cmd).output()
    };

    output
}

#[derive(Clone)]
pub struct HueAppState {
    pub hue_config_controller: Arc<RwLock<HueConfigController>>,
    pub device_req_client: reqwest::Client,
    // Device flow should be parse -> push to list -> e.g call LightDevice.setColor
    //                                             -> e.g call LightDevice.status -> Returns JSON in order to build HueDeviceMap
    // TODO: Rewrite Responses.rs in order to follow this flow
    /* Lifetime of devices_list:
        Init -> Empty -> Parse Config -> Fill
        On Push New Device -> Push to Vec -> Update Light Config -> Save Light Config
    */
    // pub device_list: Arc<Vec<Box<dyn hue_api::devices::LightDevice + Send + Sync>>>,
}

impl HueAppState {
    pub fn get_controller_read(&self) -> std::sync::RwLockReadGuard<HueConfigController> {
        self.hue_config_controller.read().unwrap()
    }

    pub fn get_controller_write(&self) -> std::sync::RwLockWriteGuard<HueConfigController> {
        self.hue_config_controller.write().unwrap()
    }
}
