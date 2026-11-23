// There must only be one test in this file because of the thread-unsafety of
// environment variable acceses.

#![cfg(target_os = "linux")]

use sd_notify::NotifyState;
use std::env;
use std::os::linux::net::SocketAddrExt;
use std::os::unix::net::{SocketAddr, UnixDatagram};

#[test]
fn notify_abstract() {
    let name = format!("sd-notify-test-{}", std::process::id());
    let addr = SocketAddr::from_abstract_name(name.as_bytes()).unwrap();
    let sock = UnixDatagram::bind_addr(&addr).unwrap();

    env::set_var("NOTIFY_SOCKET", format!("@{}", name));

    sd_notify::notify(&[NotifyState::Ready]).unwrap();

    let mut buf = [0; 1024];
    let len = sock.recv(&mut buf).unwrap();
    assert_eq!(&buf[..len], b"READY=1\n");
}
