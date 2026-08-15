fn main() {
    let mut server = skyhook::Server::new().unwrap();
    println!("ip: {}", server.ip().unwrap());
    loop {
        match server.listen() {
            Ok((kind, addr)) => {
                eprintln!("log: {}: {}", addr, kind);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
