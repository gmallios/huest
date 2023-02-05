use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::hue_api::types::Config::BridgeConfig;

use super::{
    responses::BackupResponse, InternetServices, Portalstate, Swupdate, Swupdate2, Whitelist,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct HueV1ConfigurationResponse {
    pub backup: BackupResponse,
    pub datastoreversion: String,
    pub dhcp: bool,
    pub factorynew: bool,
    pub internetservices: InternetServices,
    pub linkbutton: bool,
    pub modelid: String,
    pub portalconnection: String,
    pub portalservices: bool,
    pub portalstate: Portalstate,
    pub proxyaddress: String,
    pub proxyport: i64,
    pub replacesbridgeid: Option<String>,
    pub swupdate: Swupdate,
    pub swupdate2: Swupdate2,
    pub zigbeechannel: i64,
    pub apiversion: String,
    pub bridgeid: String,
    pub ipaddress: String,
    pub netmask: String,
    pub gateway: String,
    pub mac: String,
    pub name: String,
    pub swversion: String,
    pub timezone: String,
    #[serde(rename = "UTC")]
    pub utc: String,
    pub localtime: String,
    pub whitelist: Whitelist,
}

impl From<&BridgeConfig> for HueV1ConfigurationResponse {
    fn from(config: &BridgeConfig) -> Self {
        HueV1ConfigurationResponse {
            bridgeid: config.bridgeid.to_owned(),
            mac: config.mac.to_owned(),
            name: config.name.to_owned(),
            ipaddress: config.ipaddress.to_owned(),
            netmask: config.netmask.to_owned(),
            gateway: config.gateway.to_owned(),
            timezone: config.timezone.to_owned(),
            swversion: config.swversion.to_owned(),
            apiversion: config.apiversion.to_owned(),
            whitelist: Whitelist::from(&config.hue_users),
            utc: chrono::offset::Utc::now()
                .format("%Y-%m-%dT%H:%M:%S")
                .to_string(),
            localtime: chrono::offset::Local::now()
                .format("%Y-%m-%dT%H:%M:%S")
                .to_string(),
            ..Default::default()
        }
    }
}

impl Default for HueV1ConfigurationResponse {
    fn default() -> HueV1ConfigurationResponse {
        HueV1ConfigurationResponse {
            backup: BackupResponse::default(),
            name: "Huest Bridge".to_string(),
            modelid: "BSB002".to_string(),
            bridgeid: "".to_string(),
            internetservices: InternetServices::default(),
            factorynew: false,
            datastoreversion: "126".to_string(),
            replacesbridgeid: None,
            zigbeechannel: 15,
            mac: "".to_owned(),
            dhcp: false,
            ipaddress: "".to_string(),
            netmask: "".to_string(),
            gateway: "".to_string(),
            proxyaddress: "".to_string(),
            proxyport: 0,
            utc: "".to_string(),
            localtime: "".to_string(),
            timezone: "".to_string(),
            whitelist: Whitelist(BTreeMap::new()),
            swversion: "".to_string(),
            apiversion: "1.52.0".to_string(),
            swupdate: Swupdate::default(),
            swupdate2: Swupdate2::default(),
            linkbutton: false,
            portalservices: false,
            portalconnection: "".to_string(),
            portalstate: Portalstate::default(),
        }
    }
}
