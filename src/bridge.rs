use clap::Parser;
use std::sync::{Mutex};

#[derive(Clone)]
pub struct BridgeParams {
    pub mac_address: String,
    pub bind_ip: String,
    pub port: u16,
    pub https: bool,
}


// Preferably not be mutated after initialization. 
lazy_static! {
    pub static ref BRIDGE_PARAMS: Mutex<BridgeParams> = Mutex::new(BridgeParams {
        mac_address: mac_address::get_mac_address().unwrap().unwrap().to_string(),
        bind_ip: "0.0.0.0".to_string(),
        port: 6565,
        https: false
    });
}

pub fn config_get_mac_addr() -> String {
    return BRIDGE_PARAMS.lock().unwrap().mac_address.clone();
}

// #[derive(Parser,Debug)]
// #[clap(author, version, about, long_about = None)]
// struct BridgeArguements {
//     #[clap(long, value_parser)]
//     https: bool
// }
