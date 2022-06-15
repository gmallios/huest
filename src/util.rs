use std::{path::Path, fs::{self, File}, io::{self, Error}};
use serde::de;

const CONFIG_PATH_PREFIX: &str = "config/";

pub fn mac_addr_to_serial_number(mac_addr: &str) -> String {
    let mac_addr = mac_addr.replace(":", "");
    format!(
        "{}fffe{}",
        mac_addr[..6].to_string(),
        mac_addr[6..].to_string()
    )
}

pub fn mac_addr_to_bridge_id(mac_addr: &str) -> String {
    mac_addr_to_serial_number(mac_addr).to_uppercase()
}

pub fn load_config<T: de::DeserializeOwned>(filename: &str) -> T where T: std::default::Default + serde::Serialize{
    let path = format!("{}/{}", CONFIG_PATH_PREFIX, &filename);
    if !Path::new(&path).exists() {
        //File::create(format!("{}/{}", "", filename)).expect_err(format!("Can't create {}", filename).as_str());
        save_config(&filename, T::default()).expect(format!("Can't save {}", filename).as_str());
        return load_config(filename);
    }

    let file = match fs::read_to_string(&path) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to read config file: {}", &path);
            std::process::exit(1);
        }
    };

    let config: T = match serde_yaml::from_str(&file){
        Ok(config) => config,
        Err(_) => {
            //TODO: Create new config and rename bad one to .bad 
            println!("Failed to parse config file: {}", &path);
            std::process::exit(1);
        }
    };

    return config;
}

pub fn save_config<T>(filename: &str, config: T) -> Result<(), std::io::Error> where T: serde::Serialize {
    let path = format!("{}/{}", CONFIG_PATH_PREFIX, &filename);
    let config_str = match serde_yaml::to_string(&config) {
        Ok(s) => s,
        Err(e) => {
            println!("Error: {}", e);
            return Err(Error::new(
                io::ErrorKind::Other,
                "Error: Failed to serialize  config",
            ));
        }
    };
    fs::write(path, config_str)
}
#[cfg(test)]
mod util_config {
    use serde::{Serialize, Deserialize};
    use crate::util::CONFIG_PATH_PREFIX;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct ExampleConfig {
        pub name: String,
        pub age: u8,
    }

    impl Default for ExampleConfig {
        fn default() -> Self {
            ExampleConfig {
                name: "Default Value".to_string(),
                age: 69,
            }
        }
    }

    #[test]
    fn test_config_flow() {
        // Test both saving and loading config
        let example_config = ExampleConfig {
            name: "John".to_string(),
            age: 30,
        };

        super::save_config("test.yaml", &example_config).unwrap();
        let loaded_config: ExampleConfig = super::load_config::<ExampleConfig>("test.yaml");
        assert_eq!(example_config, loaded_config);
        std::fs::remove_file(format!("{}/test.yaml", CONFIG_PATH_PREFIX)).unwrap();
    }
}