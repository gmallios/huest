use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use self::wled::WLEDProtoData;

use super::types::{
    internal::InternalDevice,
    v1::light::{HueV1LightItemResponse, HueV1LightSimpleItemResponse},
};

pub mod wled;

pub struct Device {
    pub ip: String,
    pub port: u16,
    pub mac: String,
    pub name: String,
}

#[async_trait]
pub trait LightDevice: Send + Sync {
    async fn new(device: &InternalDevice, client: reqwest::Client) -> Self
    where
        Self: Sized;
    async fn get_v1_state(&self) -> HueV1LightItemResponse;
    async fn get_v1_state_simple(&self) -> HueV1LightSimpleItemResponse;
    async fn get_v2_state(&self);
    async fn send_color(&self, color: XYColorData);
    async fn set_brightness(&self, brightness: u8);
    // async fn refetch_state(&self);
    fn get_ip(&self) -> String;
    fn get_port(&self) -> u16;
    fn get_mac(&self) -> String;
    fn get_name(&self) -> String;
    fn get_v1_id(&self) -> u8;
    fn get_v2_id(&self) -> String;
}

pub struct RGBColorData {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct XYColorData {
    pub X: f32,
    pub Y: f32,
}

// Used for specifying protocol-specific config params
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(untagged)]
pub enum ProtocolCfg {
    #[default]
    None,
    WLEDProtocolCfg(WLEDProtoData),
}
