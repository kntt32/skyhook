use super::*;

#[derive(Debug)]
pub struct Server {
    data: Vec<u8>,
    socket: net::TcpListener,
}

impl Server {
    pub fn new() -> io::Result<Self> {
        let socket = net::TcpListener::bind("0.0.0.0:3845")?;
        Ok(Self {
            data: Vec::new(),
            socket,
        })
    }

    pub fn ip(&self) -> io::Result<net::Ipv4Addr> {
        ip::get_local_ip()
    }

    pub fn listen(&mut self) -> io::Result<(PacketKind, net::SocketAddr)> {
        let timeout = time::Duration::from_millis(10000);
        let (mut stream, addr) = self.socket.accept()?;
        stream.set_write_timeout(Some(timeout))?;
        stream.set_read_timeout(Some(timeout))?;
        let mut code = 0;
        stream.read_exact(slice::from_mut(&mut code))?;
        let Some(kind) = PacketKind::from_code(code) else {
            return Err(io::Error::new(io::ErrorKind::Other, "invalid response"));
        };
        let mut data = Vec::new();
        stream.read_to_end(&mut data)?;
        let request_packet = Packet { kind, data };
        match request_packet.kind {
            PacketKind::Get => {
                stream.write_all(&[PacketKind::Get.to_code()])?;
                stream.write_all(&self.data)?;
            }
            PacketKind::Set => {
                self.data = request_packet.data;
                stream.write_all(&[PacketKind::Set.to_code()])?;
            }
        }
        Ok((request_packet.kind, addr))
    }
}
