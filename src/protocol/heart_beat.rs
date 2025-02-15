
use crate::error::PacketError;
use crate::network::ToBytes;
use crate::packet::Packet;
use crate::protocol::preregister_connection::PACKET_PREREGISTER_CONNECTION;
use chrono::prelude::*;

pub const PACKET_HEART_BEAT_RESPONSE: i32 = 109;
#[derive(Debug, PartialEq)]
pub struct HeartBeatPacket {
    pub ping_number: i64,
    pub unknown_byte: u8,
    pub unknown_byte2:u8,
}
impl HeartBeatPacket {
    pub fn new(ping : i64) -> Self {
        Self {
            ping_number: ping,
            unknown_byte: 1,
            unknown_byte2: 58,
        }
    }
    pub fn from_packet(packet: &mut Packet) -> Result<Self, PacketError> {
        let _total_length = packet.read_i32()?;
        let packet_type = packet.read_i32()?;
        if packet_type != PACKET_HEART_BEAT_RESPONSE {
            return Err(PacketError::InvalidPacketType);
        }
        let ping_number = packet.read_i64()?;
        let unknown_byte = packet.read_byte()?;
        let unknown_byte2 = packet.read_byte()?;
        Ok(Self{
            ping_number,
            unknown_byte,
            unknown_byte2,
        })
    }
}
impl ToBytes for HeartBeatPacket {
    fn to_bytes(&self) -> Result<Vec<u8>, PacketError> {
        let mut inner = Packet::new();
        inner.write_i32(PACKET_HEART_BEAT_RESPONSE)?;
        inner.write_i64(self.ping_number)?;
        inner.write_byte(self.unknown_byte)?;
        inner.write_byte(self.unknown_byte2)?;

        let mut final_packet = Packet::new();
        let total_length = inner.payload.len() as i32 -4;
        final_packet.write_i32(total_length)?;
        final_packet.write_bytes(&inner.payload)?;

        Ok(final_packet.payload)
    }
}