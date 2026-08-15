use std::io;
use std::net;
use std::ptr;

pub fn get_local_ip() -> io::Result<net::Ipv4Addr> {
    let mut ifaddrs_ptr: *mut libc::ifaddrs = ptr::null_mut();
    unsafe {
        if libc::getifaddrs(&mut ifaddrs_ptr) != 0 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "failed to get local ip",
            ));
        }
    }
    let mut current = ifaddrs_ptr;
    let mut found_ip = None;
    while !current.is_null() {
        unsafe {
            let ifaddr = *current;
            let flags = ifaddr.ifa_flags;
            let is_up = (flags & libc::IFF_UP as u32) != 0;
            let is_loopback = (flags & libc::IFF_LOOPBACK as u32) != 0;
            if is_up && !is_loopback && !ifaddr.ifa_addr.is_null() {
                let family = (*ifaddr.ifa_addr).sa_family as i32;
                if family == libc::AF_INET {
                    let sockaddr_in = ifaddr.ifa_addr as *const libc::sockaddr_in;
                    let sin_addr = (*sockaddr_in).sin_addr;
                    let ip_bytes = sin_addr.s_addr.to_be_bytes();
                    found_ip = Some(net::Ipv4Addr::from(ip_bytes));
                    break;
                }
            }
            current = ifaddr.ifa_next;
        }
    }
    unsafe {
        libc::freeifaddrs(ifaddrs_ptr);
    }
    found_ip.ok_or_else(|| io::Error::new(io::ErrorKind::Other, "failed to get local ip"))
}
