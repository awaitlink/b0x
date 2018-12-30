//! Deals with IP addresses (`IpAddr`)

use super::*;
use std::net::IpAddr;

/// Run all passes with this `IpAddr`.
pub fn run(ip: &IpAddr, config: &Config) {
    pass_sequence!(ip, config, "ip"; general, specific, conversions);
}

/// Prints general information about this `IpAddr`
pub fn general(ip: &IpAddr) {
    match ip {
        IpAddr::V4(_) => info!("type", "IPv4"),
        IpAddr::V6(_) => info!("type", "IPv6"),
    }

    info!("unspecified?", ip.is_unspecified());
    info!("loopback?", ip.is_loopback());
    info!("multicast?", ip.is_multicast());
}

/// Prints information about this `IpAddr` specific to its version
pub fn specific(ip: &IpAddr) {
    match ip {
        IpAddr::V4(v) => {
            info!("octets", format!("{:?}", v.octets()));
            info!("private?", v.is_private());
            info!("link-local?", v.is_link_local());
            info!("broadcast?", v.is_broadcast());
            info!("documentation?", v.is_documentation());
        },
        IpAddr::V6(v) => {
            info!("octets", format!("{:?}", v.octets()));
            info!("segments", format!("{:?}", v.segments()));
        },
    }
}

/// Prints this `IpAddr`, converted to other type (IPv4 -> IPv6, IPv6 -> IPv4)
pub fn conversions(ip: &IpAddr) {
    match ip {
        IpAddr::V4(v) => {
            info!("ipv6-compatible", v.to_ipv6_compatible());
            info!("ipv6-mapped", v.to_ipv6_mapped());
        },
        IpAddr::V6(v) => {
            match v.to_ipv4() {
                Some(v) => info!("ipv4", v),
                None => na!("ipv4")
            };
        },
    }
}
