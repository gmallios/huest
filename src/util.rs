pub fn mac_addr_to_serial_number(mac_addr: &str) -> String {
    let mac_addr = mac_addr.replace(':', "");
    format!("{}fffe{}", &mac_addr[..6], &mac_addr[6..])
}

pub fn mac_addr_to_bridge_id(mac_addr: &str) -> String {
    mac_addr_to_serial_number(mac_addr).to_uppercase()
}

pub mod config {
    use serde::de;
    use std::{
        fs::{self},
        io::{self, Error},
        path::Path,
    };
    pub const CONFIG_PATH_PREFIX: &str = "config/";

    pub fn create_config_dir_if_not_exists() -> Result<(), Error> {
        let config_dir = Path::new(CONFIG_PATH_PREFIX);
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)?;
        }
        Ok(())
    }

    pub fn load_config<T: de::DeserializeOwned>(filename: &str) -> T
    where
        T: std::default::Default + serde::Serialize,
    {
        let path: &str = &format!("{}/{}", CONFIG_PATH_PREFIX, &filename);
        if !Path::new(&path).exists() {
            //File::create(format!("{}/{}", "", filename)).expect_err(format!("Can't create {}", filename).as_str());
            save_config(filename, &T::default())
                .unwrap_or_else(|_| panic!("Can't save {}", filename));
            return load_config(filename);
        }

        let file = match fs::read_to_string(path) {
            Ok(file) => file,
            Err(_) => {
                panic!("Failed to read config file: {}", &path);
            }
        };

        let config: T = match serde_yaml::from_str(&file) {
            Ok(config) => config,
            Err(_) => {
                panic!("Failed to parse config file: {}", &path);
                // let new_path: &str = &format!("{}/{}.bad", CONFIG_PATH_PREFIX, &filename);
                // println!("Failed to parse config file: {}", &path);
                // fs::rename(path, new_path)
                //     .unwrap_or_else(|_| panic!("Can't rename {} to {}", path, new_path));
                // save_config(filename, &T::default())
                //     .unwrap_or_else(|_| panic!("Can't save {}", filename));
                // return load_config(filename);
            }
        };

        config
    }

    pub fn save_config<T>(filename: &str, config: &T) -> Result<(), std::io::Error>
    where
        T: serde::Serialize,
    {
        let path = format!("{}/{}", CONFIG_PATH_PREFIX, &filename);
        let config_str = match serde_yaml::to_string(config) {
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
}

#[cfg(test)]
mod util_config {
    use serde::{Deserialize, Serialize};

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
    fn config_save() {
        use crate::util::config::save_config;
        let config = ExampleConfig::default();
        save_config("test_save.yaml", &config).unwrap();
        std::fs::remove_file(format!(
            "{}/test_save.yaml",
            crate::util::config::CONFIG_PATH_PREFIX
        ))
        .unwrap();
    }

    #[test]
    fn config_flow() {
        // Test both saving and loading config
        let example_config = ExampleConfig {
            name: "John".to_string(),
            age: 30,
        };

        crate::util::config::save_config("test.yaml", &example_config).unwrap();
        let loaded_config: ExampleConfig =
            crate::util::config::load_config::<ExampleConfig>("test.yaml");
        assert_eq!(example_config, loaded_config);
        std::fs::remove_file(format!(
            "{}/test.yaml",
            crate::util::config::CONFIG_PATH_PREFIX
        ))
        .unwrap();
    }
}
