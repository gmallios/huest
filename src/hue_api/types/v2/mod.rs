use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod grouped_light;
pub mod responses;

#[derive(Serialize, Debug)]
pub struct EmptyObj {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResourceResponse<T> {
    pub errors: Vec<Error>,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Error {
    pub description: String,
}

#[derive(Serialize, Debug)]
pub struct GroupedLight {
    alert: serde_json::Value, /* Hardcoded Value */
    color: EmptyObj,
    dimming: EmptyObj,
    dimming_delta: EmptyObj,
    dynamics: EmptyObj,
    id: String,
    id_v1: String,
    on: OnObj, /* True if any of lights added to the bridge is on. AKA is anything on */
    #[serde(rename = "type")]
    rtype: Rtypes,
}

impl Default for GroupedLight {
    fn default() -> Self {
        let id = Uuid::new_v5(
            &Uuid::NAMESPACE_URL,
            format!("{}group", "examplegroup0usedfortesting").as_bytes(),
        );
        Self {
            alert: json!(
                {
                    "action_values": [
                        "breathe"
                    ]
                }
            ),
            color: EmptyObj {},
            dimming: EmptyObj {},
            dimming_delta: EmptyObj {},
            dynamics: EmptyObj {},
            id: id.to_string(),
            id_v1: "/groups/0".to_string(),
            on: OnObj { on: false },
            rtype: Rtypes::grouped_light,
        }
    }
}
/* Should be from Internal Group type */
// impl From<&BridgeConfig> for GroupedLight {
//     fn from(bridge_config: &BridgeConfig) -> Self {
//         Self {
//             alert: json!(
//                 r#"
//                     "action_values": [
//                         "breathe"
//                     ]
//             "#
//             ),
//             color: EmptyObj {},
//             dimming: EmptyObj {},
//             dimming_delta: EmptyObj {},
//             dynamics: EmptyObj {},
//             id: bridge_config.bridge_id.clone(),
//             id_v1: bridge_config.bridge_id.clone(),
//             on: OnObj { on: false },
//             rtype: "grouped_light".to_string(),
//         }
//     }
// }

#[derive(Serialize, Debug)]
pub struct OnObj {
    on: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Timezone {
    pub time_zone: String,
}

/* Better naming needed */
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SimpleResource {
    pub rid: String,
    pub rtype: Rtypes,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Rtypes {
    device,
    bridge,
    bridge_home,
    room,
    zone,
    light,
    button,
    relative_rotary,
    temperature,
    light_level,
    motion,
    entertainment,
    grouped_light,
    device_power,
    zigbee_bridge_connectivity,
    zigbee_connectivity,
    zgp_connectivity,
    zigbee_device_discovery,
    homekit,
    matter,
    matter_fabric,
    scene,
    entertainment_configuration,
    public_image,
    auth_v1,
    behavior_script,
    behavior_instance,
    geofence,
    geofence_client,
    geolocation,
    smart_scene,
}
