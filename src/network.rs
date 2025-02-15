use crate::error::PacketError;
use crate::protocol::preregister_connection::PreregisterConnectionPacket;
use std::io::Write;
use crate::packet::Packet;
use std::net::TcpStream;
use std::io::Read;

pub fn send(stream: &mut TcpStream, packet: &PreregisterConnectionPacket) -> Result<(), PacketError> {
    let packet_data = packet.to_bytes()?;
    stream
        .write_all(&packet_data)
        .map_err(|e| PacketError::IoError(e.to_string()))?;
    stream
        .flush()
        .map_err(|e| PacketError::IoError(e.to_string()))?;
    Ok(())
}
