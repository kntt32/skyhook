use std::env;
use std::io;
use std::io::IsTerminal;
use std::io::Read;
use std::io::Write;

fn main() {
    let ip;
    if let Some(a) = env::args().nth(1)
        && let Ok(b) = a.parse()
    {
        ip = b;
    } else if let Ok(a) = env::var("AIRHOOK")
        && let Ok(b) = a.parse()
    {
        ip = b;
    } else {
        ip = "127.0.0.1".parse().unwrap();
    }
    let packet;
    if io::stdin().is_terminal() {
        packet = sky::Packet {
            kind: sky::PacketKind::Get,
            data: Vec::new(),
        };
    } else {
        let mut data = Vec::new();
        io::stdin().read_to_end(&mut data).unwrap();
        packet = sky::Packet {
            kind: sky::PacketKind::Set,
            data,
        };
    }
    let client = sky::Client::new(ip);
    match client.access(packet) {
        Ok(packet) => {
            if packet.kind == sky::PacketKind::Get {
                io::stdout().write_all(&packet.data).unwrap();
            }
        }
        Err(e) => {
            eprintln!("error: {}", e);
        }
    }
}
