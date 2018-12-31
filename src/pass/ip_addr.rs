//! Deals with IP addresses (`IpAddr`)

use super::*;
use std::net::IpAddr;

/// Run all passes with this `IpAddr`.
pub fn run(ip: &IpAddr, config: &Config) {
    pass_sequence!(ip, config, "ip"; information, conversions);
}

/// Prints information about this `IpAddr`
pub fn information(ip: &IpAddr) {
    let other = "other".cyan().bold().to_string();

    match ip {
        IpAddr::V4(v) => {
            info!("type", "IPv4");

            info!(
                "kind",
                match v {
                    _ if v.is_unspecified() => "unspecified",
                    _ if v.is_loopback() => "loopback",
                    _ if v.is_multicast() => "multicast",

                    _ if v.is_private() => "private",
                    _ if v.is_link_local() => "link-local",
                    _ if v.is_broadcast() => "broadcast",
                    _ if v.is_documentation() => "documentation",

                    _ => &other,
                }
            );

            info!("octets", format!("{:?}", v.octets()));
        }
        IpAddr::V6(v) => {
            info!("type", "IPv6");

            info!(
                "kind",
                match v {
                    _ if v.is_unspecified() => "unspecified",
                    _ if v.is_loopback() => "loopback",
                    _ if v.is_multicast() => "multicast",

                    _ => &other,
                }
            );

            info!("octets", format!("{:?}", v.octets()));
            info!("segments", format!("{:?}", v.segments()));
        }
    }
}

/// Prints this `IpAddr`, converted to other type (IPv4 -> IPv6, IPv6 -> IPv4)
pub fn conversions(ip: &IpAddr) {
    match ip {
        IpAddr::V4(v) => {
            info!("ipv6 compatible", v.to_ipv6_compatible());
            info!("ipv6 mapped", v.to_ipv6_mapped());
        }
        IpAddr::V6(v) => {
            match v.to_ipv4() {
                Some(v) => info!("ipv4", v),
                None => na!("ipv4"),
            };
        }
    }
}
