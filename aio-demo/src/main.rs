//! aio study
//!
//! https://www.youtube.com/watch?v=_3LpJ6I-tzc

use std::collections::HashMap;
use std::io::{Read, Result, Write};
use std::net::TcpListener;
use std::os::unix::prelude::AsRawFd;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7000")?;
    listener.set_nonblocking(true)?;

    let fd = listener.as_raw_fd();
    let mut poll_fds = vec![libc::pollfd {
        fd,
        events: libc::POLLIN,
        revents: 0,
    }];

    let mut handlers: HashMap<i32, Box<dyn Fn()>> = HashMap::new();
    handlers.insert(
        fd,
        Box::new(move || {
            let (client, addr) = listener.accept().unwrap();
            handle_connection(client, addr);
        }),
    );

    loop {
        let num_events = wait(&mut poll_fds)?;
        if num_events > 0 {
            for poll_fd in &poll_fds {
                if poll_fd.revents & libc::POLLIN != 0 {
                    if let Some(handler) = handlers.get(&poll_fd.fd) {
                        handler();
                    }
                }
            }
        }
    }
}

fn handle_connection(client: std::net::TcpStream, addr: std::net::SocketAddr) {
    println!("{} connected", addr);
    let mut buf = [0u8; 1024];
    let mut client = client;
    loop {
        let n = client.read(&mut buf).unwrap();
        if n == 0 {
            break;
        }
        client.write_all(&buf[..n]).unwrap();
    }
}

macro_rules! syscall {
    ($fn:ident $args:tt) => {{
        let res = unsafe { libc::$fn $args };
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }};
}

fn wait(fds: &mut [libc::pollfd]) -> Result<usize> {
    //
    loop {
        match syscall!(poll(
            fds.as_mut_ptr() as *mut libc::pollfd,
            fds.len() as libc::nfds_t,
            -1
        )) {
            Ok(n) => break Ok(n as usize),
            Err(e) if e.raw_os_error() == Some(libc::EAGAIN) => continue,
            Err(e) => return Err(e),
        }
    }
}
