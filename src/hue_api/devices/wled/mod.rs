use crate::hue_api::devices::{LightDevice, XYColorData};
use crate::hue_api::types::internal::{DeviceProtosData, InternalDevice};
use crate::hue_api::types::v1::light::{HueV1LightItemResponse, HueV1LightSimpleItemResponse};
use serde::{Deserialize, Serialize};

pub struct WLEDDevice {
    pub id: String,
    pub name: String,
    pub ip: String,
    pub port: u16,
    internal_state: WLEDInternalState
}

struct WLEDInternalState {

}

impl LightDevice for WLEDDevice {
    fn new(device: &InternalDevice) -> Self
    where
        Self: Sized,
    {
        match device.proto_data {
            DeviceProtosData::WLEDProtoData(ref data) => WLEDDevice {
                id: device.id.clone(),
                name: device.name.clone(),
                ip: data.ip.clone(),
                port: 80,
                internal_state: todo!(),
            },
            _ => {
                panic!("Invalid protocol data for WLEDDevice");
            }
        }
    }

    fn get_v1_state(&self) -> HueV1LightItemResponse {
        unimplemented!()
    }

    fn get_v1_state_simple(&self) -> HueV1LightSimpleItemResponse {
        unimplemented!()
    }

    fn get_v2_state(&self) {
        unimplemented!()
    }

    fn get_ip(&self) -> String {
        self.ip.clone()
    }

    fn get_port(&self) -> u16 {
        self.port
    }

    fn get_mac(&self) -> String {
        unimplemented!()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn send_color(&self, _color: XYColorData) {
        unimplemented!()
    }

    fn set_brightness(&self, _brightness: u8) {
        unimplemented!()
    }

    fn get_v1_id(&self) -> u8 {
        unimplemented!()
    }

    fn get_v2_id(&self) -> String {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WLEDProtoData {
    pub ip: String,
    pub backlight: bool,
    pub model: String,
    pub led_count: u32,
    pub mdns_name: String,
    pub mac: String,
    pub segment_id: u32,
}
