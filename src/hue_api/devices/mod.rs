use serde::{Serialize, Deserialize};

use self::WLED::{WLEDDevice, WLEDProtocolCfg};

pub mod WLED;

pub struct Device {
    pub ip: String,
    pub port: u16,
    pub mac: String,
    pub name: String,
}


// PoC for "converting" device_map to responding device structs
// Could not get dynamic dispatch to work, so this is a suboptimal way to do it 
pub struct LightDeviceList {
    pub wled: Vec<WLEDDevice>,
}

pub trait LightDevice {
    fn new(&self) -> Box<dyn LightDevice>;
    fn get_ip(&self) -> String;
    fn get_port(&self) -> u16;
    fn get_mac(&self) -> String;
    fn get_name(&self) -> String;
    fn send_color(&self, color: XYColorData);
    fn set_brightness(&self, brightness: u8);
}

struct RGBColorData {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

struct XYColorData {
    pub X: f32,
    pub Y: f32,
}



// Used for specifying protocol-specific config params
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(untagged)]
pub enum ProtocolCfg {
    #[default]
    None,
    WLEDProtocolCfg(WLEDProtocolCfg)
}
