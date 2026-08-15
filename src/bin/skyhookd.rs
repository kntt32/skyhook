fn main() {
    let mut server = sky::Server::new().unwrap();
    println!("ip: {}", server.ip().unwrap());
    loop {
        match server.listen() {
            Ok((packet, addr)) => {
                eprintln!("log: {}: {}", addr, packet);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
