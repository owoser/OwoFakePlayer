use crate::error::PacketError;
use crate::packet::Packet;

pub const PACKET_PREREGISTER_CONNECTION: i32 = 161;

#[derive(Debug, PartialEq)]
pub struct RegisterConnectionPacket {
    pub server_id: String,
    pub protocol_version: i32,
    pub game_version: i32,
    pub another_game_version: i32,
    pub pkg_name: String,
    pub network_server_id: String,
    pub server_key: i32,
    pub color: i32,
    zero: i32,
}
impl RegisterConnectionPacket {
    pub fn new() -> Self {
        Self {
            server_id: "com.corrodinggames.rts".to_string(),
            protocol_version: 2,
            game_version: 176,
            another_game_version: 176,
            pkg_name: "com.corrodinggames.rts.java".to_string(),
            network_server_id: String::new(),
            server_key: 0, //6
            color: 0,  //7
            zero: 0,
        }
    }
    pub fn from_packet(packet: &mut Packet) -> Result<Self, PacketError> {
        let _total_length = packet.read_i32()?;
        let packet_type = packet.read_i32()?;
        if packet_type != PACKET_PREREGISTER_CONNECTION {
            return Err(PacketError::InvalidPacketType);
        }

        let server_id = packet.read_string()?;
        let protocol_version = packet.read_i32()?;
        let game_version = packet.read_i32()?;
        let another_game_version = packet.read_i32()?;
        let pkg_name = packet.read_string()?;
        let network_server_id = packet.read_string()?;
        let server_key = packet.read_i32()?;
        let color = packet.read_i32()?;
        let zero = packet.read_i32()?;
        //packet.read_bytes(2)?;

        Ok(Self {
            server_id,
            protocol_version,
            game_version,
            another_game_version,
            pkg_name,
            network_server_id,
            server_key,
            color,
            zero,
        })
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>, PacketError> {
        let mut inner = Packet::new();
        inner.write_i32(PACKET_PREREGISTER_CONNECTION)?;
        inner.write_string(&self.server_id)?;
        inner.write_i32(self.protocol_version)?;
        inner.write_i32(self.game_version)?;
        inner.write_i32(self.another_game_version)?;
        inner.write_string(&self.pkg_name)?;
        inner.write_string(&self.network_server_id)?;
        inner.write_i32(self.server_key)?;
        inner.write_i32(self.color)?;
        inner.write_i32(0)?;

        let mut final_packet = Packet::new();
        let total_length = inner.payload.len() as i32 -4;
        final_packet.write_i32(total_length)?;
        final_packet.write_bytes(&inner.payload)?;

        Ok(final_packet.payload)
    }

}



