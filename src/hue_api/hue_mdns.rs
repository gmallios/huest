use std::any::Any;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use zeroconf::prelude::*;
use zeroconf::{MdnsService, ServiceRegistration, ServiceType, TxtRecord};
use log::{debug, error, log_enabled, info, Level};


#[derive(Default, Debug)]
pub struct Context {
    service_name: String,
}

pub fn start_hue_mdns() {
    let mut service = MdnsService::new(ServiceType::new("hue", "tcp").unwrap(), 80);
    let mut txt_record = TxtRecord::new();
    let context: Arc<Mutex<Context>> = Arc::default();

    let bridge_id = "26B8F8";

    // TODO: Use proper bridge id
    txt_record.insert("name", &format!("Philips Hue - {}", bridge_id)).unwrap();
    service.set_name(&format!("Philips Hue - {}", bridge_id));

    service.set_registered_callback(Box::new(on_service_registered));
    service.set_context(Box::new(context));
    service.set_txt_record(txt_record);

    let event_loop = service.register().unwrap();

    loop {
        // calling `poll()` will keep this service alive
        event_loop.poll(Duration::from_millis(1500)).unwrap();
    }
}

fn on_service_registered(
    result: zeroconf::Result<ServiceRegistration>,
    context: Option<Arc<dyn Any>>,
) {
    let service = result.unwrap();

    // println!("Service registered: {:?}", service);

    let context = context
        .as_ref()
        .unwrap()
        .downcast_ref::<Arc<Mutex<Context>>>()
        .unwrap()
        .clone();

    context.lock().unwrap().service_name = service.name().clone();
    info!("Hue mDNS service registered: {:?}", service);
    // println!("Context: {:?}", context);

}