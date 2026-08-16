use super::*;

#[derive(Debug)]
pub struct Client {
    addr: net::SocketAddrV4,
}

impl Client {
    pub fn new(ip: net::Ipv4Addr) -> Self {
        Self {
            addr: net::SocketAddrV4::new(ip, 0x0f05),
        }
    }

    pub fn access(&self, packet: Packet) -> io::Result<Packet> {
        let timeout = time::Duration::from_millis(10000);
        let mut stream = net::TcpStream::connect(self.addr)?;
        stream.set_write_timeout(Some(timeout))?;
        stream.set_read_timeout(Some(timeout))?;
        stream.write_all(&[packet.kind.to_code()])?;
        stream.write_all(&packet.data)?;
        stream.shutdown(net::Shutdown::Write)?;
        let mut code = 0;
        stream.read_exact(slice::from_mut(&mut code))?;
        let Some(kind) = PacketKind::from_code(code) else {
            return Err(io::Error::new(io::ErrorKind::Other, "invalid response"));
        };
        let mut data = Vec::new();
        stream.read_to_end(&mut data)?;
        Ok(Packet { kind, data })
    }
}
