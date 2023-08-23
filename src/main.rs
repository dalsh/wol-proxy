mod mac_address;
mod config;
mod signal_handler;

use std::net::{UdpSocket};

use crate::config::AppConfig;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app_config = AppConfig::init();
    signal_handler::start().expect("Could not start signal handler");

    let socket = UdpSocket::bind(app_config.local_listen_address)?;
    println!("Listening on {}", socket.local_addr()?);
    socket.set_broadcast(true)?;
    let mut buf = vec![0; 1024];
    loop {
        let (size, _) = socket.recv_from(&mut buf)?;

        let received_data = &buf[..size];
        let received_mac = mac_address::extract_from_magic_packet(received_data);

        match received_mac {
            None => {
                println!("Got invalid MagicPacket, dropping");
            }
            Some(received_mac_address) => {
                if app_config.check_whitelist(&received_mac_address) {
                    println!("Got MagicPacket, relaying to {}", app_config.relay_address);
                    let _ = socket.send_to(received_data, app_config.relay_address)?;
                } else {
                    println!("Got MagicPacket for non-whitelisted MAC address: {}", received_mac_address);
                }
            }
        }
    }
}
