//! Deals with IP addresses (`IpAddr`)

use super::*;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Pass sequence for IP addresses (`IpAddr`).
pub struct IpAddrPasses;
impl<'a> PassSequence<'a> for IpAddrPasses {
    type T = IpAddr;
    const TY_NAME: &'static str = "ip";
    const PASSES: &'a [Pass<'a, Self::T>] = pass_sequence![information, conversions];
}

/// Returns the kind of an IPv4 address.
pub fn ip_addr_kind_ipv4(ip: &Ipv4Addr) -> Option<&'static str> {
    match ip {
        _ if ip.is_unspecified() => Some("unspecified"),
        _ if ip.is_loopback() => Some("loopback"),
        _ if ip.is_multicast() => Some("multicast"),

        _ if ip.is_private() => Some("private"),
        _ if ip.is_link_local() => Some("link-local"),
        _ if ip.is_broadcast() => Some("broadcast"),
        _ if ip.is_documentation() => Some("documentation"),

        _ => None,
    }
}

/// Returns the kind of an IPv6 address.
pub fn ip_addr_kind_ipv6(ip: &Ipv6Addr) -> Option<&'static str> {
    match ip {
        _ if ip.is_unspecified() => Some("unspecified"),
        _ if ip.is_loopback() => Some("loopback"),
        _ if ip.is_multicast() => Some("multicast"),

        _ => None,
    }
}

/// Information about the IP address.
pub fn information<'a>(input: &IpAddr) -> Vec<(&'a str, Info)> {
    let mut result = Vec::new();

    match input {
        IpAddr::V4(v) => {
            push!(result, "type", "IPv4");

            match ip_addr_kind_ipv4(&v) {
                Some(kind) => push!(result, "kind", kind),
                None => push!(result, "kind", other),
            };

            push!(result, "octets", &format!("{:?}", v.octets()));
        }
        IpAddr::V6(v) => {
            push!(result, "type", "IPv6");

            match ip_addr_kind_ipv6(&v) {
                Some(kind) => push!(result, "kind", kind),
                None => push!(result, "kind", other),
            };

            push!(result, "octets", &format!("{:?}", v.octets()));
            push!(result, "segments", &format!("{:?}", v.segments()));
        }
    }

    result
}

/// Conversions of an IP address to different types (`IPv4` **->** `IPv6` or `IPv6` **->** `IPv4`).
pub fn conversions<'a>(input: &IpAddr) -> Vec<(&'a str, Info)> {
    let mut result = Vec::new();

    match input {
        IpAddr::V4(v) => {
            push!(result, "ipv6 compatible", v.to_ipv6_compatible());
            push!(result, "ipv6 mapped", v.to_ipv6_mapped());
        }
        IpAddr::V6(v) => {
            match v.to_ipv4() {
                Some(v) => push!(result, "ipv4", v),
                None => push!(result, "ipv4", n / a),
            };
        }
    }

    result
}
