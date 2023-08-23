use crate::mac_address;

use std::{env};
use std::collections::HashSet;
use std::net::{SocketAddr};

pub struct AppConfig {
    pub local_listen_address: SocketAddr,
    pub relay_address: SocketAddr,
    pub mac_addresses_whitelist: HashSet<String>,
    pub mac_addresses_whitelist_checking_enabled: bool,
}

impl AppConfig {
    pub fn check_whitelist(&self, mac_address: &String) -> bool {
        !self.mac_addresses_whitelist_checking_enabled || self.mac_addresses_whitelist.contains(mac_address.as_str())
    }

    pub fn init() -> AppConfig {
        let args: Vec<String> = env::args().collect();

        let local_listen_address: SocketAddr = args.get(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| "0.0.0.0:7".parse().expect("Could not define a listen address"));

        let relay_address: SocketAddr = args.get(2)
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| "255.255.255.255:9".parse().expect("Could not define a relay address"));

        let mac_addresses_whitelist_env = env::var("MAC_ADDRESSES_WHITELIST").unwrap_or_default();
        let (mac_addresses_whitelist, mac_addresses_whitelist_checking_enabled) = mac_address::get_clean_whitelist(mac_addresses_whitelist_env);

        AppConfig {
            local_listen_address,
            relay_address,
            mac_addresses_whitelist,
            mac_addresses_whitelist_checking_enabled,
        }
    }
}


