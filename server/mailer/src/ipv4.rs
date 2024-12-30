use crate::error::{MailerError, Result};
use futures_util::future::join_all;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::Duration;

/// Get a local IPv4 address to bind to.
/// GMail does not support sending to IPv6, hence we usually want to bind to an IPv4 interface.
///
/// # Errors
/// - If listing all addresses failed.
/// - If no suitable address could be found.
pub async fn get_local_v4() -> Result<Ipv4Addr> {
    let potential_addrs = nix::ifaddrs::getifaddrs()
        .map_err(|e| MailerError::GetAddr(e))?
        // Map to interface address
        .filter_map(|iface| iface.address)
        // Map to IPv4 address
        .filter_map(|addr| addr.as_sockaddr_in().map(|addr4| addr4.ip()))
        // Filter out loopback and link local addresses
        .filter(|addr| !addr.is_loopback() && !addr.is_link_local())
        .collect::<Vec<_>>();

    // As we cannot determine if the address can reach the internet just by the address alone, try connecting over TCP
    let connectable_addrs = join_all(potential_addrs.into_iter().map(|addr| async move {
        // Open the socket and bind it to the address under test
        let sock = tokio::net::TcpSocket::new_v4()?;
        sock.bind(SocketAddr::V4(SocketAddrV4::new(addr, 0)))?;

        // Try connecting to the internet
        match tokio::time::timeout(
            Duration::from_secs(3),
            sock.connect(SocketAddr::V4(SocketAddrV4::new(
                // Address of example.com, run by IANA so very stable
                Ipv4Addr::from([93, 184, 215, 14]),
                80,
            ))),
        )
        .await
        {
            Ok(stream_r) => stream_r.map(|_| addr),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::TimedOut, e)),
        }
    }))
    .await
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    match connectable_addrs.get(0) {
        Some(addr) => Ok(*addr),
        None => Err(MailerError::NoIpv4),
    }
}
