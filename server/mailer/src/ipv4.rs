use futures_util::future::join_all;
use nix::errno::Errno;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, ToSocketAddrs};
use std::time::Duration;
use thiserror::Error;
use tracing::trace;

#[derive(Debug, Error)]
pub enum AddressError {
    #[error("Could not retrieve addresses ({errno}): {description}")]
    GetAddresses {
        errno: Errno,
        description: &'static str,
    },
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("No local IPv4 address could be determined")]
    Undeterminable,
    #[error("Could not determine address of Google for testing")]
    NoRemote,
}

/// Get a local IPv4 address to bind to.
/// GMail does not support sending to IPv6, hence we usually want to bind to an IPv4 interface.
///
/// # Errors
/// - If listing all addresses failed.
/// - If no suitable address could be found.
pub async fn get_local_v4() -> Result<Ipv4Addr, AddressError> {
    let potential_addrs = nix::ifaddrs::getifaddrs()
        .map_err(|e| AddressError::GetAddresses {
            description: e.desc(),
            errno: e,
        })?
        // Remove loopback
        .filter_map(|iface| iface.address)
        .collect::<Vec<_>>()
        .into_iter()
        .filter_map(|addr| addr.as_sockaddr_in().map(|addr4| addr4.ip()))
        .filter(|addr| {
            let is_lo = addr.is_loopback();
            let is_ll = addr.is_link_local();

            trace!("Address {addr:?} is loopback: {is_lo}; is link_local: {is_ll}");
            !is_lo && !is_ll
        })
        .collect::<Vec<_>>();

    trace!("Trying to determine address of Google for testing");
    let remote_addrs = "google.com:443"
        .to_socket_addrs()?
        .filter(|addr| addr.is_ipv4())
        .filter_map(|addr| match addr {
            SocketAddr::V4(addr) => Some(addr),
            SocketAddr::V6(_) => None,
        })
        .collect::<Vec<_>>();
    let remote_addr = remote_addrs.first().ok_or(AddressError::NoRemote)?;
    trace!("Determined address {remote_addr:?}");

    // As we cannot determine if the address can reach the internet just by the address alone, try connecting over TCP
    let connectable_addrs = join_all(potential_addrs.into_iter().map(|addr| async move {
        let sock = tokio::net::TcpSocket::new_v4().map_err(|e| (addr, e))?;
        sock.bind(SocketAddr::V4(SocketAddrV4::new(addr, 0)))
            .map_err(|e| (addr, e))?;

        match tokio::time::timeout(
            Duration::from_secs(3),
            sock.connect(SocketAddr::V4(SocketAddrV4::new(*remote_addr.ip(), 80))),
        )
        .await
        {
            Ok(stream_r) => stream_r.map(|_| addr).map_err(|e| (addr, e)),
            Err(e) => Err((addr, std::io::Error::new(std::io::ErrorKind::TimedOut, e))),
        }
    }))
    .await
    .into_iter()
    .filter_map(|res| match res {
        Ok(v) => Some(v),
        Err((addr, e)) => {
            trace!("Address {addr:?} could not reach internet due to {e}");
            None
        }
    })
    .collect::<Vec<_>>();

    if connectable_addrs.is_empty() {
        Err(AddressError::Undeterminable)
    } else {
        Ok(connectable_addrs[0])
    }
}
