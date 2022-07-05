struct Device {
    pub ip: String,
    pub port: u16,
    pub mac: String,
    pub name: String,
}

pub trait LightDevice {
    fn new(&self) -> Self;
    fn get_ip(&self) -> String;
    fn get_port(&self) -> u16;
    fn get_mac(&self) -> String;
    fn get_name(&self) -> String;
    fn send_color(&self, color: XYColorData);
    fn set_brightness(&self, brightness: u8);
}

struct RGBColorData {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

struct XYColorData {
    pub X: f32,
    pub Y: f32,
}
