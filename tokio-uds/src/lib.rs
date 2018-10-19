//! Unix Domain Sockets for Tokio.
//!
//! This crate provides APIs for using Unix Domain Sockets with Tokio.

#![doc(html_root_url = "https://docs.rs/tokio-uds/0.2.2")]
#![deny(missing_docs, warnings, missing_debug_implementations)]

extern crate bytes;
#[macro_use]
extern crate futures;
extern crate iovec;
extern crate log;
extern crate mio;
extern crate tokio_io;
extern crate tokio_reactor;

#[cfg(unix)]
extern crate libc;
#[cfg(unix)]
extern crate mio_uds;

#[cfg(windows)]
extern crate mio_uds_windows;

mod datagram;
mod incoming;
mod listener;
mod recv_dgram;
mod send_dgram;
mod stream;
mod ucred;

#[cfg(unix)]
pub use datagram::UnixDatagram;
pub use incoming::Incoming;
pub use listener::UnixListener;
#[cfg(unix)]
pub use recv_dgram::RecvDgram;
#[cfg(unix)]
pub use send_dgram::SendDgram;
pub use stream::{UnixStream, ConnectFuture};
#[cfg(unix)]
pub use ucred::UCred;
