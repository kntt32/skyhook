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

    pub fn listen(&mut self) -> io::Result<(Packet, net::SocketAddr)> {
        let timeout = time::Duration::from_millis(10000);
        let (mut stream, addr) = self.socket.accept()?;
        stream.set_write_timeout(Some(timeout))?;
        stream.set_read_timeout(Some(timeout))?;
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf)?;
        let Some(packet) = Packet::from_vec(buf) else {
            return Err(io::Error::new(io::ErrorKind::Other, "invalid request"));
        };
        match packet.kind {
            PacketKind::Get => {
                stream.write_all(
                    &Packet {
                        kind: PacketKind::Get,
                        data: self.data.clone(),
                    }
                    .to_vec(),
                )?;
            }
            PacketKind::Set => {
                self.data.clone_from(&packet.data);
                stream.write_all(
                    &Packet {
                        kind: PacketKind::Set,
                        data: Vec::new(),
                    }
                    .to_vec(),
                )?;
            }
        }
        Ok((packet, addr))
    }
}
