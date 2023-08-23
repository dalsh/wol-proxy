use std::collections::HashSet;
use regex::Regex;

pub fn validate_format(mac_address: &str) -> bool {
    let mac_regex = Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap();
    if mac_regex.is_match(mac_address) {
        true
    } else {
        eprintln!("Warning: '{}' is not a valid MAC address", mac_address);
        false
    }
}

pub fn get_clean_whitelist(mac_addresses_whitelist_raw: String) -> (HashSet<String>, bool) {
    if mac_addresses_whitelist_raw.is_empty() {
        eprintln!("Warning: MAC_ADDRESSES_WHITELIST is not set or empty. Whitelist checking disabled");
        (HashSet::new(), false)
    } else {
        let mac_addresses_whitelist: HashSet<String> = mac_addresses_whitelist_raw.split(',')
            .map(|s| String::from(s.trim()))
            .filter(|s| {
                validate_format(s)
            })
            .collect();

        let mac_addresses_whitelist_checking_enabled = !mac_addresses_whitelist.is_empty();
        if !mac_addresses_whitelist_checking_enabled {
            eprintln!("Warning: MAC_ADDRESSES_WHITELIST contained invalid mac addresses. Whitelist checking disabled");
        } else {
            println!("Whitelisted MAC addresses:");
            for mac_address in &mac_addresses_whitelist {
                println!("- {}", mac_address);
            }
        }

        (mac_addresses_whitelist, mac_addresses_whitelist_checking_enabled)
    }
}

pub fn extract_from_magic_packet(packet: &[u8]) -> Option<String> {
    if packet.len() < 7 || !packet.starts_with(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff]) {
        return None; // Not a valid magic packet
    }

    let candidate = &packet[6..12];
    let packet_chunks = packet[12..].chunks(6);

    for window in packet_chunks {
        if window != candidate {
            return None;
        }
    }

    // We have a proper magic packet, return the MAC address
    Some(candidate.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join(":"))
}
