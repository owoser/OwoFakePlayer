use crate::packet::Packet;
use crate::error::PacketError;
use crate::network::ToBytes;

pub const PACKET_PREREGISTER_CONNECTION: i32 = 160;

#[derive(Debug, PartialEq)]
pub struct PreregisterConnectionPacket {
    pub package_name: String,
    pub protocol_version: i32,
    pub game_version: i32,
    pub another_game_version: i32,
    pub relay_id: String,
    pub nickname: String,
    pub locale: String,
}

impl PreregisterConnectionPacket {
    pub fn new() -> Self {
        Self {
            package_name: "com.corrodinggames.rts".to_string(),
            protocol_version: 4,
            game_version: 176,
            another_game_version: 2,
            relay_id: String::new(),
            nickname: "wanan".to_string(),
            locale: "zh".to_string(),
        }
    }

    pub fn from_packet(packet: &mut Packet) -> Result<Self, PacketError> {
        let _total_length = packet.read_i32()?;
        let packet_type = packet.read_i32()?;
        if packet_type != PACKET_PREREGISTER_CONNECTION {
            return Err(PacketError::InvalidPacketType);
        }

        let package_name = packet.read_string()?;
        let protocol_version = packet.read_i32()?;
        let game_version = packet.read_i32()?;
        let another_game_version = packet.read_i32()?;

        let relay_id = if protocol_version >= 2 {
            packet.read_is_string()?
        } else {
            String::new()
        };

        let nickname = if protocol_version >= 3 {
            packet.read_string()?
        } else {
            String::new()
        };

        let locale = packet.read_string()?;

        packet.read_bytes(2)?;

        Ok(Self {
            package_name,
            protocol_version,
            game_version,
            another_game_version,
            relay_id,
            nickname,
            locale,
        })
    }

}
impl ToBytes for PreregisterConnectionPacket {
    fn to_bytes(&self) -> Result<Vec<u8>, PacketError> {
        let mut inner = Packet::new();
        inner.write_i32(PACKET_PREREGISTER_CONNECTION)?;
        inner.write_string(&self.package_name)?;
        inner.write_i32(self.protocol_version)?;
        inner.write_i32(self.game_version)?;
        inner.write_i32(self.another_game_version)?;

        if self.protocol_version >= 2 {
            inner.write_is_string(&self.relay_id)?;
        }

        if self.protocol_version >= 3 {
            inner.write_string(&self.nickname)?;
        }

        inner.write_string(&self.locale)?;
        inner.write_i16(0)?;

        let mut final_packet = Packet::new();
        let total_length = inner.payload.len() as i32 -4;
        final_packet.write_i32(total_length)?;
        final_packet.write_bytes(&inner.payload)?;

        Ok(final_packet.payload)
    }
}