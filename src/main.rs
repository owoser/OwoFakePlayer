mod error;
mod packet;
mod protocol;
mod network;
use std::net::{Shutdown, TcpStream};
use error::PacketError;
use protocol::preregister_connection::PreregisterConnectionPacket;
use network::send;

fn main() -> Result<(), PacketError> {
    let packet = PreregisterConnectionPacket::new();
    println!("{:?}", packet.to_bytes().unwrap());

    let ser_ip = "192.168.1.5:5123";
    let mut stream = match TcpStream::connect(ser_ip) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error connecting to server: {}", e);
            return Ok(());
        }
    };

    if let Err(e) = send(&mut stream, &packet) {
        eprintln!("发送数据包失败: {}", e);
        stream
            .shutdown(Shutdown::Both)
            .map_err(|e| PacketError::IoError(e.to_string()))?;
        return Ok(());
    }

    stream
        .shutdown(Shutdown::Both)
        .map_err(|e| PacketError::IoError(e.to_string()))?;
    Ok(())
}