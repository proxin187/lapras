use std::net::{TcpStream, Ipv4Addr, IpAddr, SocketAddr};
use std::time::Duration;
use std::io::Write;


pub fn send(ip: Ipv4Addr, raw: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect_timeout(&SocketAddr::new(IpAddr::V4(ip), 80), Duration::from_secs(2))?;

    stream.write_all(raw.as_bytes())?;

    Ok(())
}


