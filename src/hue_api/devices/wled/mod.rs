use std::sync::RwLock;

use crate::hue_api::devices::wled::responses::WLEDStateResponse;
use crate::hue_api::devices::{LightDevice, XYColorData};
use crate::hue_api::types::internal::{DeviceProtosData, InternalDevice};
use crate::hue_api::types::v1::light::{
    HueV1LightItemResponse, HueV1LightSimpleItemResponse, HueV1NewLightState, ModelIDData, State,
};
use crate::hue_api::util::xy_to_rgb;
use async_trait::async_trait;

use serde::{Deserialize, Serialize};

use self::responses::Seg;

mod responses;

/* This will get used to create a WLEDDevice Instance */
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WLEDProtoData {
    pub ip: String,
    pub backlight: bool,
    pub model: String,
    pub led_count: u16,
    pub mdns_name: String,
    pub mac: String,
    pub segment_id: u16,
}

pub struct WLEDDevice {
    pub id_v1: u8,
    pub id_v2: String,
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub segment_id: u16,
    state: RwLock<State>,
    client: reqwest::Client,
}
#[async_trait]
trait WLED {
    async fn get_state_and_seg(&self) -> (WLEDStateResponse, Seg);
}

#[async_trait]
impl WLED for WLEDDevice {
    async fn get_state_and_seg(&self) -> (WLEDStateResponse, Seg) {
        let resp: WLEDStateResponse = self
            .client
            .get(&format!("http://{}:{}/json/state", self.ip, self.port))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        let our_seg = resp
            .clone()
            .seg
            .into_iter()
            .find(|seg| seg.id == self.segment_id)
            .unwrap();
        return (resp, our_seg);
    }
}
// TODO: Optimistically update state
#[async_trait]
impl LightDevice for WLEDDevice {
    async fn new(device: &InternalDevice, client: reqwest::Client) -> Self
    where
        Self: Sized,
    {
        match device.proto_data {
            DeviceProtosData::WLEDProtoData(ref proto_data) => WLEDDevice {
                id_v1: device.id_v1,
                id_v2: device.id_v2.clone(),
                name: device.name.clone(),
                ip: proto_data.ip.clone(),
                port: 80,
                segment_id: proto_data.segment_id,
                client,
                state: RwLock::new(State::default()),
            },
            _ => {
                panic!("Invalid protocol data for WLEDDevice");
            }
        }
    }

    async fn refetch_state(&self) {
        // TODO: Compare old state to new state and only update if changed, this will prevent unnecessary write locks 
        *self.state.write().unwrap() = State::from(self.get_state_and_seg().await);
    }

    fn get_v1_state(&self) -> HueV1LightItemResponse {
        let modelid_data = ModelIDData::LST002_V1();
        HueV1LightItemResponse {
            name: self.name.clone(),
            modelid: crate::hue_api::types::internal::ModelIDs::LST002,
            swversion: modelid_data.swversion,
            state: self.state.read().unwrap().clone(),
            ltype: modelid_data.ltype,
            capabilities: modelid_data.capabilities,
            swupdate: modelid_data.swupdate,
        }
    }

    fn get_v1_state_simple(&self) -> HueV1LightSimpleItemResponse {
        let modelid_data = ModelIDData::LST002_V1();
        HueV1LightSimpleItemResponse {
            name: self.name.clone(),
            modelid: crate::hue_api::types::internal::ModelIDs::LST002,
            swversion: modelid_data.swversion,
            ltype: modelid_data.ltype,
            state: self.state.read().unwrap().clone(),
        }
    }

    async fn set_v1_state(&self, new_state: HueV1NewLightState) {
        println!("Setting state: {:?}", new_state);
        let cmd = WLEDDeviceStateCommand {
            on: new_state.on,
            bri: new_state.bri,
            seg: vec![SegCommand {
                id: self.segment_id as u8,
                on: new_state.on,
                bri: new_state.bri.clone(),
                col: new_state.xy.map(|xy| vec![xy_to_rgb(xy.0, xy.1, 50)]), /* Bri - Adjust White Point */
            }],
        };
        self.client
            .put(&format!("http://{}:{}/json", self.ip, self.port))
            .json(&cmd)
            .send()
            .await
            .unwrap();
    }

    fn is_on(&self) -> bool {
        self.state.read().unwrap().on
    }

    async fn get_v2_state(&self) {
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

    async fn send_color(&self, _color: XYColorData) {
        unimplemented!()
    }

    async fn set_brightness(&self, _brightness: u8) {
        unimplemented!()
    }

    fn get_v1_id(&self) -> u8 {
        self.id_v1
    }

    fn get_v2_id(&self) -> String {
        unimplemented!()
    }
}

#[derive(Serialize)]
struct WLEDDeviceStateCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bri: Option<u8>,
    pub seg: Vec<SegCommand>,
}

#[derive(Serialize)]
struct SegCommand {
    pub id: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bri: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub col: Option<Vec<(u8, u8, u8)>>, /* R-G-B */
}
