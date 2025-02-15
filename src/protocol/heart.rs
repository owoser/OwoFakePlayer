use tokio::io::AsyncReadExt;
use crate::error::PacketError;
use crate::packet::Packet;
pub const PACKET_HEART_BEAT: i32 = 108;
#[derive(Debug, PartialEq)]
pub struct HeartPacket {
    pub ping_number: i64,
    pub end_byte: u8,
}
impl HeartPacket {
    pub fn new(ping_number :i64) -> Self {
        Self {
            ping_number,
            end_byte: 0,
        }
    }
    pub fn from_packet(packet: &mut Packet) -> Result<Self, PacketError> {
        let _total_length = packet.read_i32()?;
        let packet_type = packet.read_i32()?;
        if packet_type != PACKET_HEART_BEAT {
            return Err(PacketError::InvalidPacketType);
        }
        let ping_number = packet.read_i64()?;
        let end_byte = packet.read_byte()?;
        Ok(Self {
            ping_number,
            end_byte,
        })
    }
}