use bridge::config_get_mac_addr;
use clap::command;
use rocket::{
    config::CipherSuite,
    config::TlsConfig,
    http::{
        tls::rustls::{cipher_suite::TLS13_AES_256_GCM_SHA384, Tls13CipherSuite},
        Status,
    },
    response::{content, status},
    Config,
};
use ssdp::start_ssdp_broadcast;
use std::{
    default, fs,
    net::Ipv4Addr,
    process::Command,
    sync::{Arc, Mutex},
    thread,
};

use hue_api::hue_config_controller::{HueConfigController, HueConfigControllerState};
use hue_api::hue_mdns::start_hue_mdns;
use rocket::futures::future;

#[macro_use]
extern crate rocket;
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

#[rocket::main]
async fn main() {
    println!("Starting Hue Bridge...");
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
            println!("SSL certificates generated!");
        }
        Err(_) => {
            println!("Failed to generate SSL certificates!");
            std::process::exit(1);
        }
    }

    thread::spawn(|| start_ssdp_broadcast());
    thread::spawn(|| start_hue_mdns());

    let api_state = HueConfigControllerState {
        hue_config_controller: Arc::new(Mutex::new(HueConfigController::new())),
    };

    let default = Config::release_default();
    let cert_set = CipherSuite::TLS_V13_SET;
    let tls_config = TlsConfig::from_paths("./ssl/cert.pem", "./ssl/private.pem")
        .with_preferred_server_cipher_order(true)
        .with_ciphers([CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256]);
    let https_config = Config {
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        tls: Some(tls_config),
        port: 443,
        ..default.clone()
    };

    let http_config = Config {
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        port: 80,
        ..default
    };
    // let https = rocket::custom(&https_config)
    //     .manage(api_state.clone())
    //     .mount("/", routes![hello, description_xml])
    //     .mount("/api", hue_api::hue_routes())
    //     .launch();
    
    // Current solution is to use flask to proxy https 
    let a = Command::new("python3").arg("reverseproxy.py").spawn();

    let http = rocket::custom(&http_config)
        .manage(api_state)
        .mount("/", routes![hello, description_xml])
        .mount("/api", hue_api::hue_routes())
        .launch().await;

    // let _pair = future::try_join(http, https).await;
}

#[get("/")]
async fn hello() -> &'static str {
    "Hello world!"
}

#[get("/description.xml")]
async fn description_xml() -> status::Custom<content::RawXml<String>> {
    status::Custom(
        Status::Ok,
        content::RawXml(fs::read_to_string("static/description.xml").unwrap()),
    )
}

fn gen_ssl_cert() -> Result<std::process::ExitStatus, std::io::Error> {
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
        .spawn()
        .expect("failed to execute process")
        .wait()
}
