use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

use super::Config::HueUser;

pub mod responses;
pub mod datastore;
pub mod configuration;
pub mod light;
pub mod group;

#[derive(Serialize, Deserialize, Clone)]
pub struct Portalstate {
    pub signedon: bool,
    pub incoming: bool,
    pub outgoing: bool,
    pub communication: String,
}

impl Default for Portalstate {
    fn default() -> Self {
        Portalstate {
            signedon: false,
            incoming: false,
            outgoing: false,
            communication: "disconnected".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Swupdate {
    pub updatestate: i64,
    pub url: String,
    pub text: String,
    pub notify: bool,
    pub lights: Vec<String>,
    pub sensors: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Swupdate2 {
    pub autoinstall: Swupdate2Autoinstall,
    pub bridge: Swupdate2Bridge,
    pub checkforupdate: bool,
    pub lastchange: String,
    pub state: String,
}

impl Default for Swupdate2 {
    fn default() -> Self {
        Swupdate2 {
            autoinstall: Swupdate2Autoinstall::default(),
            bridge: Swupdate2Bridge::default(),
            checkforupdate: false,
            lastchange: "2021-01-01T00:00:00".to_string(),
            state: "unknown".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Swupdate2Autoinstall {
    pub on: bool,
    pub updatetime: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Swupdate2Bridge {
    pub lastinstall: String,
    pub state: String,
}

impl Default for Swupdate2Bridge {
    fn default() -> Self {
        Swupdate2Bridge {
            lastinstall: "2021-01-01T00:00:00".to_string(),
            state: "noupdates".to_string(),
        }
    }
}

impl Default for Swupdate2Autoinstall {
    fn default() -> Self {
        Swupdate2Autoinstall {
            on: true,
            updatetime: "T14:00:00".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InternetServices {
    internet: String,
    remoteaccess: String,
    time: String,
    swupdate: String,
}

impl Default for InternetServices {
    fn default() -> Self {
        InternetServices {
            internet: "disconnected".to_string(),
            remoteaccess: "disconnected".to_string(),
            time: "disconnected".to_string(),
            swupdate: "disconnected".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WhitelistItem {
    #[serde(rename = "last use date")]
    pub last_use_date: String,
    #[serde(rename = "create date")]
    pub create_date: String,
    pub name: String,
}


/* Maybe flatten */
#[derive(Serialize, Deserialize, Clone)]
pub struct Whitelist(BTreeMap<String, WhitelistItem>);

impl From<&BTreeMap<u8, HueUser>> for Whitelist {
    fn from(users: &BTreeMap<u8, HueUser>) -> Self {
        let mut whitelist = Whitelist(BTreeMap::new());
        for (_, user) in users {
            whitelist.0.insert(
                user.client_key.clone(),
                WhitelistItem {
                    last_use_date: user.date_last_connected.clone(),
                    create_date: user.date_created.clone(),
                    name: user.devicetype.clone(),
                },
            );
        }
        whitelist
    }
}