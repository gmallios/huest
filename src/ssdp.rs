use std::{
    net::{self, Ipv4Addr, UdpSocket},
    str::FromStr,
    thread,
    time::Duration,
};



static SSDP_INTERVAL: u64  = 1500;

// https://github.com/rustasync/team/issues/81
// I don't know if this is working yet
pub fn start_ssdp_broadcast() {
    let ssdp_socket = match UdpSocket::bind("0.0.0.0:1900") {
        Ok(socket) => socket,
        Err(e) => {
            println!("Failed to bind to SSDP socket: {}", e);
            return;
        }
    };
    let ssdp_msg = format!("HTTP/1.1 200 OK\r\n CACHE-CONTROL: max-age=100\r\n EXT:\r\n LOCATION: http://{}:80/description.xml\r\n SERVER: FreeRTOS/6.0.5, UPnP/1.0, IpBridge/0.1\r\n ST: uuid:0FDD7736-722C-4995-89F2-ABCDEF000000\r\n USN: uuid:0FDD7736-722C-4995-89F2-ABCDEF000000\r\n \r\n", crate::bridge::BRIDGE_PARAMS.lock().unwrap().local_ip);
    ssdp_socket
        .join_multicast_v4(
            &Ipv4Addr::from_str("239.255.255.250").unwrap(),
            &Ipv4Addr::UNSPECIFIED,
        )
        .unwrap();


    let msg = ssdp_msg.as_bytes();
    let mut buf = [0; 1024];
    for _ in 0..10 {
        let (number_of_bytes, src_addr) =
                ssdp_socket.recv_from(&mut buf).expect("Didn't receive data");
        // ssdp_socket
        //     .send_to(&msg, "239.255.255.250:1900")
        //     .expect("Failed to send SSDP message");
        ssdp_socket
            .send_to(&msg, src_addr)
            .expect("Failed to send SSDP message");
        thread::sleep(Duration::from_millis(SSDP_INTERVAL));
    }
}
