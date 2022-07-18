// Use clap to parse command line arguments
//use default_net::{self, gateway, Interface};
use log::warn;
use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct BridgeParams {
   // pub https: bool,
    pub https_port: u16,
    pub http_port: u16,
   // pub mac_address: String,
   // pub bind_ip: String,
   // pub local_ip: String,
    //pub gateway: String,
    //pub iface: Interface,
}

// Source of truth?
// Should be used exclusively for reads after initialization.
// lazy_static! {
//     pub static ref BRIDGE_PARAMS: Arc<RwLock<BridgeParams>> = Arc::new(RwLock::new(BridgeParams {
//         initialized: false,
//         https: true,
//         https_port: 443,
//         http_port: 80,
//         mac_address: String::from("A"),
//         bind_ip: "0.0.0.0".to_string(),
//         local_ip: String::new(),
//         gateway: String::new(),
//         iface: default_net::get_default_interface().unwrap(),
//     }));
// }

pub static BRIDGE_PARAMS: Lazy<Arc<RwLock<BridgeParams>>> = Lazy::new(|| {
    let params = BridgeParams {
       // https: true,
        https_port: 443,
        http_port: 80,
        // mac_address: String::from("A"),
        // bind_ip: "0.0.0.0".to_string(),
        // local_ip: String::new(),
        // gateway: String::new(),
        // iface: default_net::get_default_interface().unwrap(),
    };

    Arc::new(RwLock::new(params))
});

fn init() {
    let mut bridge_params = BRIDGE_PARAMS.write().unwrap();

   // bridge_params.iface = get_iface();
    // bridge_params.local_ip = bridge_params.iface.ipv4.first().unwrap().clone().addr.to_string();

    // if let Some(mac) = &bridge_params.iface.mac_addr {
    //     bridge_params.mac_address = mac.to_string();
    // } else {
    //     warn!("No MAC address found for interface {}", bridge_params.iface.name);
    //     std::process::exit(1);
    // }

    // if let Some(gateway) = &bridge_params.iface.gateway {
    //     bridge_params.gateway = gateway.ip_addr.to_string();
    // } else {
    //     warn!("No gateway found for interface {}", bridge_params.iface.name);
    //     std::process::exit(1);
    // }

   // bridge_params.initialized = true;
}

// fn get_iface() -> Interface {
//     // TODO: Check parameters for ovveride else use default_net::get_default_interface()
//     let iface = match default_net::get_default_interface() {
//         Ok(iface) => iface,
//         Err(_) => {
//             warn!("Failed to get default interface!");
//             std::process::exit(1);
//         }
//     };
//     iface
// }

/* Utility functions */
pub fn get_mac_addr() -> String {
    String::from("B")
   // return BRIDGE_PARAMS.read().unwrap().mac_address.to_string();
}

pub fn get_local_ip() -> String {
    String::from("A")
   // return BRIDGE_PARAMS.read().unwrap().local_ip.to_string();
}
