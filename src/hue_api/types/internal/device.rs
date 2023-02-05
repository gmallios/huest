use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::hue_api::devices::wled::WLEDProtoData;

pub type InternalDeviceMap = std::collections::BTreeMap<u8, InternalDevice>;

#[derive(Serialize, Deserialize, Debug)]
pub struct InternalDevice {
    pub id: String,
    pub id_v2: String,
    pub name: String,
    pub modelid: ModelIDs,
    pub device_type: DeviceTypes,
    pub proto: DeviceProtos,
    pub proto_data: DeviceProtosData,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DeviceTypes {
    Light,
    Unknown,
}

#[derive(Serialize, Deserialize, Display, Debug)]
pub enum ModelIDs {
    LCT001,
    LCT015,
    LST002,
    LWB010,
    LCX004,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DeviceProtos {
    WLED,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(untagged)]
pub enum DeviceProtosData {
    #[default]
    None,
    WLEDProtoData(WLEDProtoData),
}
