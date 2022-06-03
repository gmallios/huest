use std::sync::{Mutex, Arc};
use bridge::config_get_mac_addr;
use clap::command;
use hue_api::hue_config_controller::{HueConfigController, HueConfigControllerState};

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

    // Generate SSL Certificates
    match gen_ssl_cert() {
        Ok(_) => {
            println!("SSL certificates generated!");
        },
        Err(_) => {
            println!("Failed to generate SSL certificates!");
            std::process::exit(1);
        }
    }


    let api_state = HueConfigControllerState {
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



fn gen_ssl_cert() -> Result<std::process::ExitStatus, std::io::Error> {
    use std::process::Command;
    
    let mac_addr = config_get_mac_addr().replace(":", "");
    let serial = format!("{}fffe{}",mac_addr[..6].to_string(), mac_addr[6..].to_string());
    let decimal_serial = format!("{}", u64::from_str_radix(&serial, 16).unwrap());
    let cmd = format!("{} req -new -days 3650 -config ssl/openssl.conf  -nodes -x509 -newkey  ec -pkeyopt ec_paramgen_curve:P-256 -pkeyopt ec_param_enc:named_curve   -subj \"/C=NL/O=Philips Hue/CN={}\" -keyout ssl/private.pem -out ssl/cert.pem -set_serial {}",OPENSSL_PATH,serial,decimal_serial);
    Command::new("/bin/sh")
        .arg("-c")
        .arg(cmd)
        .spawn()
        .expect("failed to execute process")
        .wait()

}
