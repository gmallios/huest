use serde::{Serialize, Deserialize};

use super::LightDevice;

pub struct WLEDDevice {
    pub id: String,
    pub name: String,
    pub ip: String,
    pub port: u16,
}

impl LightDevice for WLEDDevice {
    
    fn new(&self) -> Box<dyn LightDevice> {
        Box::new(WLEDDevice {
            id: todo!(),
            name: todo!(),
            ip: todo!(),
            port: todo!(),
        })
    }

    fn get_ip(&self) -> String {
        todo!()
    }

    fn get_port(&self) -> u16 {
        todo!()
    }

    fn get_mac(&self) -> String {
        todo!()
    }

    fn get_name(&self) -> String {
        todo!()
    }

    fn send_color(&self, color: super::XYColorData) {
        todo!()
    }

    fn set_brightness(&self, brightness: u8) {
        todo!()
    }

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WLEDProtocolCfg {
    #[serde(rename = "ip")]
    ip: String,
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "backlight")]
    backlight: bool,
    #[serde(rename = "model")]
    model: String,
    #[serde(rename = "ledCount")]
    led_count: i64,
    #[serde(rename = "mdns_name")]
    mdns_name: String,
    #[serde(rename = "mac")]
    mac: String,
    #[serde(rename = "segmentId")]
    segment_id: i64,
}

