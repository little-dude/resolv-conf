//! The crate simply parses `/etc/resolv.conf` file and creates a config object
//!
//!

#[macro_use]
extern crate quick_error;

mod grammar;

pub use grammar::ParseError;

/// A network, that is an IP address and the mask
#[derive(Clone, Debug)]
pub enum Network {
    // Address netmask
    V4(Ipv4Addr, Ipv4Addr),
    V6(Ipv6Addr, Ipv6Addr),
}

use std::net::{Ipv4Addr, Ipv6Addr};

/// Represent an IP address
#[derive(Debug, Clone)]
pub enum Ip {
    /// Represent an IPv4 address
    V4(Ipv4Addr),
    /// Represent an IPv6 and its scope identifier, if any
    V6(Ipv6Addr, Option<String>),
}

/// Encompasses the nameserver configuration
///
/// Currently the options and defaults match those of linux/glibc
#[derive(Default, Clone, Debug)]
pub struct Config {
    /// List of nameservers
    pub nameservers: Vec<Ip>,
    /// List of suffixes to append to name when it doesn't contain ndots
    pub search: Vec<String>,
    /// List of preferred addresses
    pub sortlist: Vec<Network>,
    /// Enable DNS resolve debugging
    pub debug: bool,
    /// Number of dots in name to try absolute resolving first (default 1)
    pub ndots: u32,
    /// Dns query timeout (default 5 [sec])
    pub timeout: u32,
    /// Number of attempts to resolve name if server is inaccesible (default 2)
    pub attempts: u32,
    /// Round-robin selection of servers (default false)
    pub rotate: bool,
    /// Don't check names for validity (default false)
    pub no_check_names: bool,
    /// Try AAAA query before A
    pub inet6: bool,
    /// Use reverse lookup of ipv6 using bit-label format described instead
    /// of nibble format
    pub ip6_bytestring: bool,
    /// Do ipv6 reverse lookups in ip6.int zone instead of ip6.arpa
    /// (default false)
    pub ip6_dotint: bool,
    /// Enable dns extensions described in RFC 2671
    pub edns0: bool,
    /// Don't make ipv4 and ipv6 requests simultaneously
    pub single_request: bool,
    /// Use same socket for the A and AAAA requests
    pub single_request_reopen: bool,
    /// Don't resolve unqualified name as top level domain
    pub no_tld_query: bool,
    /// Force using TCP for DNS resolution
    pub use_vc: bool,
}

impl Config {
    pub fn new() -> Self {
        Config {
            ndots: 1,
            timeout: 5,
            attempts: 2,
            ..Default::default()
        }
    }
    pub fn parse<T: AsRef<[u8]>>(buf: T) -> Result<Config, grammar::ParseError> {
        grammar::parse(buf.as_ref())
    }
}
