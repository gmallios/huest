use std::collections::HashMap;

use serde::Serialize;
use uuid::Uuid;

use crate::hue_api::{hue_routes::V2ApiUserGuard, types::Config::BridgeConfig};

use super::{EmptyObj, Rtypes, SimpleResource, Timezone};
// # client_key: c4349881dfa34bf0b6703607342d6650 Original

#[derive(Serialize, Debug)]
pub struct BridgeDevice {
    id: String,
    id_v1: String,
    #[serde(rename = "type")]
    rtype: Rtypes,
    metadata: HashMap<String, String>,
    product_data: BridgeDeviceProductData,
    services: Vec<SimpleResource>,
}
#[derive(Serialize, Debug)]
pub struct BridgeDeviceProductData {
    certified: bool,
    manufacturer_name: String,
    model_id: String,
    product_archetype: String,
    product_name: String,
    software_version: String,
}

impl From<&BridgeConfig> for BridgeDevice {
    fn from(config: &BridgeConfig) -> Self {
        let id = Uuid::new_v5(
            &Uuid::NAMESPACE_URL,
            format!("{}-device", config.bridgeid).as_bytes(),
        )
        .to_string();

        let metadata: HashMap<String, String> = HashMap::from([
            ("archetype".to_string(), "bridge_v2".to_string()),
            ("name".to_string(), config.name.clone()),
        ]);

        let product_data = BridgeDeviceProductData {
            certified: true,
            manufacturer_name: "Signify Netherlands B.V.".to_string(),
            model_id: "BSB002".to_string(),
            product_archetype: "bridge_v2".to_string(),
            product_name: "Philips Hue".to_string(),
            software_version: format!("{}{}", &config.apiversion[..5], &config.swversion),
        };

        let services = vec![
            SimpleResource {
                rid: Uuid::new_v5(
                    &Uuid::NAMESPACE_URL,
                    format!("{}-bridge", config.bridgeid).as_bytes(),
                )
                .to_string(),
                rtype: Rtypes::bridge,
            },
            SimpleResource {
                rid: Uuid::new_v5(
                    &Uuid::NAMESPACE_URL,
                    format!("{}-zigbee_connectivity", config.bridgeid).as_bytes(),
                )
                .to_string(),
                rtype: Rtypes::zigbee_connectivity,
            },
            SimpleResource {
                rid: Uuid::new_v5(
                    &Uuid::NAMESPACE_URL,
                    format!("{}-entertainment", config.bridgeid).as_bytes(),
                )
                .to_string(),
                rtype: Rtypes::entertainment,
            },
        ];

        BridgeDevice {
            id,
            rtype: Rtypes::device,
            id_v1: "".to_string(),
            metadata,
            product_data,
            services,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Metadata {
    archetype: String,
    name: String,
}

#[derive(Serialize, Debug)]
pub struct BridgeResource {
    pub bridge_id: String,
    pub id: String,
    pub id_v1: String,
    pub identify: EmptyObj, /* Unknown */
    pub owner: SimpleResource,
    pub time_zone: Timezone,
    #[serde(rename = "type")]
    pub btype: String,
}

impl From<&BridgeConfig> for BridgeResource {
    fn from(config: &BridgeConfig) -> Self {
        let id = Uuid::new_v5(
            &Uuid::NAMESPACE_URL,
            format!("{}bridge", config.bridgeid).as_bytes(),
        );
        let rid = Uuid::new_v5(
            &Uuid::NAMESPACE_URL,
            format!("{}device", config.bridgeid).as_bytes(),
        );
        BridgeResource {
            bridge_id: config.bridgeid.clone().to_lowercase(),
            id: id.to_string(),
            id_v1: "".to_string(),
            identify: EmptyObj {},
            owner: SimpleResource {
                rid: rid.to_string(),
                rtype: Rtypes::device,
            },
            time_zone: Timezone {
                time_zone: config.timezone.clone(),
            },
            btype: "bridge".to_string(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Homekit {
    id: String,
    status: String,
    status_values: Vec<String>,
    #[serde(rename = "type")]
    htype: Rtypes,
}

impl From<&BridgeConfig> for Homekit {
    fn from(config: &BridgeConfig) -> Self {
        let id = Uuid::new_v5(
            &Uuid::NAMESPACE_URL,
            format!("{}-homekit", config.bridgeid).as_bytes(),
        );
        Homekit {
            id: id.to_string(),
            status: "unpaired".to_string(),
            status_values: vec![
                "paired".to_string(),
                "pairing".to_string(),
                "unpaired".to_string(),
            ],
            htype: Rtypes::homekit,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Zigbee {
    id: String,
    id_v1: String,
    status: String,
    #[serde(rename = "type")]
    rtype: Rtypes,
}

impl From<&BridgeConfig> for Zigbee {
    fn from(config: &BridgeConfig) -> Self {
        let id = Uuid::new_v5(
            &Uuid::NAMESPACE_URL,
            format!("{}-zigbee_connectivity", config.bridgeid).as_bytes(),
        );
        Zigbee {
            id: id.to_string(),
            id_v1: "".to_string(),
            status: "connected".to_string(),
            rtype: Rtypes::zigbee_connectivity,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Entertainment {
    id: String,
    id_v1: String,
    proxy: bool,
    renderer: bool,
    #[serde(rename = "type")]
    rtype: Rtypes,
}

impl From<&BridgeConfig> for Entertainment {
    fn from(_config: &BridgeConfig) -> Self {
        /* Generate ID? */
        // let id = Uuid::new_v5(
        //   &Uuid::NAMESPACE_URL,
        //   format!("{}-entertainment", config.bridgeid).as_bytes(),
        // );
        let id = "57a9ebc9-406d-4a29-a4ff-42acee9e9be7";
        Entertainment {
            id: id.to_string(),
            id_v1: "".to_string(),
            proxy: true,
            renderer: false,
            rtype: Rtypes::entertainment,
        }
    }
}

/* v2BridgeHome() */
#[derive(Serialize, Debug)]
pub struct BridgeHome {
    id: String,
    id_v1: String,
    #[serde(rename = "type")]
    rtype: Rtypes,
    children: Vec<String>,         /* TODO: Fill with Devices,Groups,Sensors */
    grouped_services: Vec<String>, /* TODO: Fill with Lights in Group 0 */
    services: Vec<String>,         /* TODO: Fill with Lights,Sensors */
}

impl From<&BridgeConfig> for BridgeHome {
    fn from(config: &BridgeConfig) -> Self {
        /* ID is the id_v2 of group 0 + bridge_home */
        /* TODO: Fix/Properly implement */
        let id = Uuid::new_v5(
            &Uuid::NAMESPACE_URL,
            format!("{}-home", config.bridgeid).as_bytes(),
        );
        BridgeHome {
            id: id.to_string(),
            id_v1: "/groups/0".to_string(),
            rtype: Rtypes::bridge_home,
            children: vec![],
            grouped_services: vec![],
            services: vec![],
        }
    }
}

#[derive(Serialize, Debug)]
pub struct GeofenceClient {
    id: String,
    name: String,
    #[serde(rename = "type")]
    rtype: Rtypes,
}

impl From<&V2ApiUserGuard> for GeofenceClient {
    fn from(guard: &V2ApiUserGuard) -> Self {
        let id = Uuid::new_v5(&Uuid::NAMESPACE_URL, guard.key.as_bytes());
        GeofenceClient {
            id: id.to_string(),
            name: guard.name.clone(),
            rtype: Rtypes::geofence_client,
        }
    }
}

/* TODO: REMOVE GROUP */
pub fn behavior_scripts() -> Vec<serde_json::Value> {
    let str = r#"
        [
          {
          "configuration_schema": {
            "$ref": "coming_home_config.json#"
          },
          "description": "Automatically turn your lights to choosen light states, when you arrive at home.",
          "id": "fd60fcd1-4809-4813-b510-4a18856a595c",
          "metadata": {
            "category": "automation",
            "name": "Coming home"
          },
          "state_schema": {},
          "supported_features": [],
          "trigger_schema": {
            "$ref": "trigger.json#"
          },
          "type": "behavior_script",
          "version": "0.0.1"
        },
        {
          "configuration_schema": {
            "$ref": "leaving_home_config.json#"
          },
          "description": "Automatically turn off your lights when you leave",
          "id": "0194752a-2d53-4f92-8209-dfdc52745af3",
          "metadata": {
            "category": "automation",
            "name": "Leaving home"
          },
          "state_schema": {},
          "supported_features": [],
          "trigger_schema": {
            "$ref": "trigger.json#"
          },
          "type": "behavior_script",
          "version": "0.0.1"
        },
        {
          "configuration_schema": {
            "$ref": "schedule_config.json#"
          },
          "description": "Schedule turning on and off lights",
          "id": "7238c707-8693-4f19-9095-ccdc1444d228",
          "metadata": {
            "category": "automation",
            "name": "Schedule"
          },
          "state_schema": {},
          "supported_features": [],
          "trigger_schema": {
            "$ref": "trigger.json#"
          },
          "type": "behavior_script",
          "version": "0.0.1"
        },
        {
          "configuration_schema": {
            "$ref": "timer_config.json#"
          },
          "description": "Countdown Timer",
          "id": "e73bc72d-96b1-46f8-aa57-729861f80c78",
          "metadata": {
            "category": "automation",
            "name": "Timers"
          },
          "state_schema": {
            "$ref": "timer_state.json#"
          },
          "supported_features": [],
          "trigger_schema": {
            "$ref": "trigger.json#"
          },
          "type": "behavior_script",
          "version": "0.0.1"
        },
        {
          "configuration_schema": {
            "$ref": "lights_state_after_streaming_config.json#"
          },
          "description": "State of lights in the entertainment group after streaming ends",
          "id": "7719b841-6b3d-448d-a0e7-601ae9edb6a2",
          "metadata": {
            "category": "entertainment",
            "name": "Light state after streaming"
          },
          "state_schema": {},
          "supported_features": [],
          "trigger_schema": {},
          "type": "behavior_script",
          "version": "0.0.1"
        },
        {
          "configuration_schema": {
            "$ref": "natural_light_config.json#"
          },
          "description": "Natural light during the day",
          "id": "a4260b49-0c69-4926-a29c-417f4a38a352",
          "metadata": {
            "category": "",
            "name": "Natural Light"
          },
          "state_schema": {
            "$ref": "natural_light_state.json#"
          },
          "supported_features": [],
          "trigger_schema": {
            "$ref": "natural_light_trigger.json#"
          },
          "type": "behavior_script",
          "version": "0.0.1"
        },
        {
          "id": "d242d815-2b0a-50ef-9a06-eac3e468ba74",
          "id_v1": "/sensors/1",
          "owner": {
            "rid": "d5a0192c-14eb-4a43-b2de-20b3b08d6d3d",
            "rtype": "device"
          },
          "power_state": {},
          "type": "device_power"
        }]"#;
    serde_json::from_str(str).unwrap()
}
