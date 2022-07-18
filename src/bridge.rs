use default_net::{Interface, gateway};
// Use clap to parse command line arguments
//use default_net::{self, gateway, Interface};
use log::{warn, debug};
use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct BridgeParams {
    pub https: bool,
    pub https_port: u16,
    pub http_port: u16,
    pub mac_address: String,
    pub bind_ip: String,
    pub local_ip: String,
    pub gateway_ip: String,
    pub iface: Interface,
}

// Source of truth?
// Should be used exclusively for reads after initialization.

pub static BRIDGE_PARAMS: Lazy<Arc<RwLock<BridgeParams>>> = Lazy::new(|| {

    let iface = get_iface();
    let local_ip = iface.ipv4.first().unwrap().clone().addr.to_string();
    let mut mac_address: String = String::new();
    let mut gateway_ip: String = String::new();
    if let (Some(mac), Some(gate)) = (&iface.mac_addr, &iface.gateway ) {
        mac_address = mac.to_string();
        gateway_ip = gate.ip_addr.to_string();
    } else {
        warn!("No MAC address or gateway IP found for interface {}", iface.name);
        std::process::exit(1);
    }

    let params = BridgeParams {
        https: true,
        https_port: 443,
        http_port: 80,
        bind_ip: String::from("0.0.0.0"),
        mac_address,
        local_ip,
        gateway_ip,
        iface,
    };

    debug!("params: {:?}", params);

    Arc::new(RwLock::new(params))
});


fn get_iface() -> Interface {
    // TODO: Check parameters for ovveride else use default_net::get_default_interface()
    let iface = match default_net::get_default_interface() {
        Ok(iface) => iface,
        Err(_) => {
            warn!("Failed to get default interface!");
            std::process::exit(1);
        }
    };
    iface
}

/* Utility functions */
pub fn get_mac_addr() -> String {
    return BRIDGE_PARAMS.read().unwrap().mac_address.to_string();
}

pub fn get_local_ip() -> String {
    return BRIDGE_PARAMS.read().unwrap().local_ip.to_string();
}
