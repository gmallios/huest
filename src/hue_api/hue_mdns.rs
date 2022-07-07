use log::info;


pub fn start_hue_mdns() {
    //Disable logging
    let mut builder = env_logger::Builder::new();
    builder.parse_filters("libmdns=debug");
    builder.init();

    let bridge_id = "26B8F8";
    let responder = libmdns::Responder::new().unwrap();
    let _svc = responder.register(
        "_hue._tcp".to_owned(),
        format!("Philips Hue - {}", bridge_id).to_owned(),
        80,
        &["path=/"],
    );

    info!("Hue mDNS service registered");

    loop {
        ::std::thread::sleep(::std::time::Duration::from_secs(10));
    }
}