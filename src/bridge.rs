use default_net::Interface;
// Use clap to parse command line arguments
//use default_net::{self, gateway, Interface};
use log::{error, warn};
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

// TODO: Migrate to using args
// TODO: On openwrt, the default interface is not the one we want and thus we cannot get ip/mac
pub static BRIDGE_PARAMS: Lazy<Arc<RwLock<BridgeParams>>> = Lazy::new(|| {
    let iface = get_iface();
    let local_ip = match iface.ipv4.first() {
        Some(addr) => addr.clone().addr.to_string(),
        None => {
            error!("No ipv4 address found for interface {}", iface.name);
            std::process::exit(1);
        }
    };
    let mac_address;
    let gateway_ip;

    if let (Some(mac), Some(gate)) = (&iface.mac_addr, &iface.gateway) {
        mac_address = mac.to_string();
        gateway_ip = gate.ip_addr.to_string();
    } else {
        error!(
            "No MAC address or gateway IP found for interface {}",
            iface.name
        );
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

    Arc::new(RwLock::new(params))
});

fn get_iface() -> Interface {
    // TODO: Check parameters for ovveride else use default_net::get_default_interface()
    // Use iface_name_to_iface
    let iface = match default_net::get_default_interface() {
        Ok(iface) => iface,
        Err(_) => {
            warn!("Failed to get default interface!");
            std::process::exit(1);
        }
    };
    iface
}

fn iface_name_to_iface(name: &str) -> Option<Interface> {
    default_net::get_interfaces()
        .into_iter()
        .find(|interface| interface.name == name)
}

/* Utility functions */
pub fn get_mac_addr() -> String {
    return BRIDGE_PARAMS.read().unwrap().mac_address.to_string();
}

pub fn get_local_ip() -> String {
    return BRIDGE_PARAMS.read().unwrap().local_ip.to_string();
}

pub fn get_gateway_ip() -> String {
    return BRIDGE_PARAMS.read().unwrap().gateway_ip.to_string();
}

/* I don't know about that... */
pub fn get_netmask() -> String {
    return BRIDGE_PARAMS
        .read()
        .unwrap()
        .iface
        .ipv4
        .first()
        .unwrap()
        .netmask
        .to_string();
}
