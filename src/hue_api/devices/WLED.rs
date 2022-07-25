use super::LightDevice;

pub struct WLEDDevice {
    pub id: String,
    pub name: String,
    pub ip: String,
    pub port: u16,
}

impl LightDevice for WLEDDevice {
    fn new(&self) -> Self {
        todo!()
    }
    
    fn get_ip(&self) -> String {
        todo!()
    }

    fn get_port(&self) -> u16 {
        todo!()
    }

    fn get_mac(&self) -> String {
        todo!()
    }

    fn get_name(&self) -> String {
        todo!()
    }

    fn send_color(&self, color: super::XYColorData) {
        todo!()
    }

    fn set_brightness(&self, brightness: u8) {
        todo!()
    }

    
}
