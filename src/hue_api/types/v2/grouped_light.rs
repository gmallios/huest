use serde::Serialize;

use super::{EmptyObj, Rtypes};

#[derive(Serialize, Debug)]
pub struct GroupedLightResponse {
    pub alert: Alert,
    color: EmptyObj,
    dimming_delta: EmptyObj,
    id: String,
    id_v2: String,
    on: On,
    #[serde(rename = "type")]
    rtype: Rtypes,
}
#[derive(Serialize, Debug)]
pub struct On {
    pub on: bool,
}
#[derive(Serialize, Debug)]
pub struct Alert {
    pub action_values: Vec<String>,
}

impl Default for Alert {
    fn default() -> Self {
        Alert {
            action_values: vec!["breathe".into()],
        }
    }
}