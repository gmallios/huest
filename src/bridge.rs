use clap::Parser;
use local_ip_address::local_ip;
use std::sync::{Mutex, Arc};


#[derive(Clone)]
pub struct BridgeParams {
    pub mac_address: String,
    pub bind_ip: String,
    pub port: u16,
    pub https: bool,
    pub local_ip: String
}

lazy_static! {
    pub static ref BRIDGE_PARAMS: Arc<Mutex<BridgeParams>> = Arc::new(Mutex::new(BridgeParams {
        mac_address: mac_address::get_mac_address().unwrap().unwrap().to_string(),
        bind_ip: "0.0.0.0".to_string(),
        port: 6565,
        https: false,
        local_ip: local_ip().unwrap().to_string()
    }));
}

pub fn config_get_mac_addr() -> String {
    return BRIDGE_PARAMS.lock().unwrap().mac_address.to_string();
}

// #[derive(Parser,Debug)]
// #[clap(author, version, about, long_about = None)]
// struct BridgeArguements {
//     #[clap(long, value_parser)]
//     https: bool
// }
