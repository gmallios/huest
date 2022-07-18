use actix_web::{
    get,
    middleware::{self, Logger},
    web, App, HttpResponse, HttpServer,
};
use bridge::{get_mac_addr};
use log::{error, info, warn};
use rustls::{ServerConfig, Certificate, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{
    fs::{self, File},
    process::Command,
    sync::{Arc},
    thread, io::BufReader,
};

use hue_api::{
    hue_config_controller::{HueConfigController, HueConfigControllerState},
    hue_mdns::start_hue_mdns,
    hue_routes,
};

use crate::hue_api::ssdp::start_ssdp_broadcast;

#[macro_use]
extern crate lazy_static;
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
        .init();

    info!("Starting Hue Bridge...");

    //lazy_static::initialize(&bridge::BRIDGE_PARAMS);
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

    let api_state = web::Data::new(HueConfigControllerState {
        hue_config_controller: Arc::new(std::sync::RwLock::new(HueConfigController::new())),
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


    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .app_data(api_state.clone())
            .service(description_xml)
            .service(hue_routes())
            .wrap(Logger::default())
    })
    .bind_rustls("0.0.0.0:443", load_rustls_config())?
    .bind("0.0.0.0:80")?
    .run()
    .await
}

fn load_rustls_config() -> rustls::ServerConfig {
    let config = ServerConfig::builder()
    .with_safe_defaults()
    .with_no_client_auth();

    let cert_file = &mut BufReader::new(File::open("./ssl/cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("./ssl/private.pem").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        warn!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
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
    let serial = format!(
        "{}fffe{}",
        mac_addr[..6].to_string(),
        mac_addr[6..].to_string()
    );
    let decimal_serial = format!("{}", u64::from_str_radix(&serial, 16).unwrap());
    let cmd = format!("{} req -new -days 3650 -config ssl/openssl.conf  -nodes -x509 -newkey ec -pkeyopt ec_paramgen_curve:P-256 -pkeyopt ec_param_enc:named_curve   -subj \"/C=NL/O=Philips Hue/CN={}\" -keyout ssl/private.pem -out ssl/cert.pem -set_serial {}",OPENSSL_PATH,serial,decimal_serial);

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", &cmd]).output()
    } else {
        Command::new("sh").arg("-c").arg(cmd).output()
    };

    output
}
