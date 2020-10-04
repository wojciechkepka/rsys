mod types;
#[cfg(test)]
use super::mocks::NET_DEV;
use super::SysPath;
use crate::{Error, Result};
use nix::{
    ifaddrs::getifaddrs,
    sys::socket::{InetAddr, SockAddr},
};
pub use types::*;

fn _ip(name: &str, v6: bool) -> Result<Option<String>> {
    for iface in getifaddrs()
        .map_err(|e| Error::FfiError(format!("getting ipv4 address of interface {}", name), e.to_string()))?
        .into_iter()
        .filter(|iface| iface.interface_name == name)
    {
        if let Some(addr) = iface.address {
            match addr {
                SockAddr::Inet(ip) => match ip {
                    InetAddr::V4(_) if !v6 => {
                        let s = addr.to_str();
                        // skip :<port>
                        if let Some(last_idx) = s.rfind(':') {
                            return Ok(Some(s[..last_idx].to_string()));
                        }
                        return Ok(Some(s));
                    }
                    InetAddr::V6(_) if v6 => {
                        let s = addr.to_str();
                        // skip [ ]:<port>
                        if let Some(last_idx) = s.rfind(']') {
                            return Ok(Some(s[1..last_idx].to_string()));
                        }
                        return Ok(Some(s));
                    }
                    _ => continue,
                },
                _ => continue,
            }
        }
    }
    Ok(None)
}

/// Returns a default interface. If there are no interfaces in /proc/net/arp
/// returns an empty string.
pub fn default_iface() -> Result<String> {
    if let Some(line) = SysPath::ProcNetArp.read()?.lines().nth(2) {
        if let Some(name) = line.split_ascii_whitespace().last() {
            return Ok(name.to_string());
        }
    }
    Ok("".to_string())
}

/// Returns an IPv4 address of a given iface. If the interface is not
/// found in /proc/net/arp returns "127.0.0.1"
pub fn ipv4(iface: &str) -> Result<String> {
    if let Some(ip) = _ip(&iface, false)? {
        Ok(ip)
    } else {
        Ok("".to_string())
    }
}

/// Returns an IPv6 address of a given iface.
pub fn ipv6(iface: &str) -> Result<String> {
    if let Some(ip) = _ip(&iface, true)? {
        Ok(ip)
    } else {
        Ok("".to_string())
    }
}

/// Returns a mac address of given iface
pub fn mac(iface: &str) -> Result<String> {
    Ok(SysPath::SysClassNetDev(iface).read()?.trim().to_string())
}

/// Returns a list of interfaces names.
pub fn interfaces() -> Result<Vec<String>> {
    let mut names = Vec::new();
    for entry in SysPath::SysClassNet.read_dir()? {
        if let Ok(entry) = entry {
            names.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    Ok(names)
}

/// Returns network interfaces on host os
pub fn ifaces() -> Result<Interfaces> {
    let mut ifaces = Vec::new();
    for entry in SysPath::SysClassNet.read_dir()? {
        if let Ok(entry) = entry {
            if let Some(filename) = entry.file_name().to_str() {
                ifaces.push(Interface::from_sys(filename)?);
            }
        }
    }
    Ok(Interfaces(ifaces))
}

pub fn iface(name: &str) -> Result<Option<Interface>> {
    for entry in SysPath::SysClassNet.read_dir()? {
        if let Ok(entry) = entry {
            if let Some(filename) = entry.file_name().to_str() {
                if filename == name {
                    return Ok(Some(Interface::from_sys(filename)?));
                }
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_iface_from_net_dev_line() {
        let lo = IfaceStat {
            rx_bytes: 17776656,
            rx_packets: 127989,
            rx_errs: 0,
            rx_drop: 0,
            rx_fifo: 0,
            rx_frame: 0,
            rx_compressed: 0,
            rx_multicast: 0,

            tx_bytes: 17776656,
            tx_packets: 127989,
            tx_errs: 0,
            tx_drop: 0,
            tx_fifo: 0,
            tx_frame: 0,
            tx_compressed: 0,
            tx_multicast: 0,
        };
        let enp = IfaceStat {
            rx_bytes: 482459368,
            rx_packets: 349468,
            rx_errs: 0,
            rx_drop: 0,
            rx_fifo: 0,
            rx_frame: 0,
            rx_compressed: 0,
            rx_multicast: 4785,

            tx_bytes: 16133415,
            tx_packets: 198549,
            tx_errs: 0,
            tx_drop: 0,
            tx_fifo: 0,
            tx_frame: 0,
            tx_compressed: 0,
            tx_multicast: 0,
        };
        let mut lines = NET_DEV.split('\n').skip(2);

        assert_eq!(Ok(lo), IfaceStat::from_line(lines.next().unwrap()));
        assert_eq!(Ok(enp), IfaceStat::from_line(lines.next().unwrap()))
    }
}
