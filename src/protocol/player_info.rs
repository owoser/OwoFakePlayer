use sha2::digest::consts::False;
//110 packet
use crate::packet::Packet;
use crate::error::PacketError;
use crate::packet_utils::{compute_color_for_packet, compute_key_for_packet, compute_uuid_for_packet, SerKey};
use crate::protocol::register_connection::PACKET_PREREGISTER_CONNECTION;
use uuid::Uuid;
use crate::network::ToBytes;

pub const PACKET_PLAYER_INFO: i32 = 110;

#[derive(Debug, PartialEq)]
pub struct PlayerInfoPacket {
    pub package_name: String,
    pub protocol_version: i32,
    pub game_version: i32,
    pub another_game_version: i32,
    pub nickname: String,
    pub is_password : bool,
    pub password: String,
    pub another_package_name: String,
    pub uuid_sum : String,
    pub client_units_checksum: i32,
    pub token : String,
    pub color: String,
}
impl PlayerInfoPacket {
    pub fn new(packet: &mut Packet) -> Self {
        let mut a = SerKey::new();
        SerKey::get(&mut a,packet);
        let client_uuid = Uuid::new_v4().to_string();
        Self {
            package_name: "com.corrodinggames.rts".to_string(),
            protocol_version: 5,
            game_version: 176,
            another_game_version:176,
            nickname: "wanan".to_string(),
            is_password : false,
            password: String::new(),
            another_package_name: "com.corrodinggames.rts.java".to_string(),
            uuid_sum : compute_uuid_for_packet(&*client_uuid, &*a.network_id),
            client_units_checksum: 678359601,
            token : compute_key_for_packet(a.keys),
            color: compute_color_for_packet(a.color),
        }
    }
    pub fn from_packet(packet: &mut Packet) -> Result<Self, PacketError> {
        let _total_length = packet.read_i32()?;
        let packet_type = packet.read_i32()?;
        if packet_type != PACKET_PLAYER_INFO {
            return Err(PacketError::InvalidPacketType);
        }
        let package_name = packet.read_string()?;
        let protocol_version = packet.read_i32()?;
        let game_version = packet.read_i32()?;
        let another_game_version = packet.read_i32()?;
        let nickname = packet.read_string()?;
        let is_password = packet.read_bool()?;
        let password = if is_password {
            packet.read_string()?
        } else {
            String::new()
        };
        let another_package_name = packet.read_string()?;
        let uuid_sum = packet.read_string()?;
        let client_units_checksum = packet.read_i32()?;
        let token = packet.read_string()?;
        let color = packet.read_string()?;
        //packet.read_bytes(2)?;
        Ok(Self {
            package_name,
            protocol_version,
            game_version,
            another_game_version,
            nickname,
            is_password,
            password,
            another_package_name,
            uuid_sum,
            client_units_checksum,
            token,
            color,
        })
    }
}
impl ToBytes for PlayerInfoPacket {
    fn to_bytes(&self) -> Result<Vec<u8>, PacketError> {
        let mut inner = Packet::new();
        inner.write_i32(PACKET_PLAYER_INFO)?;
        inner.write_string(&self.package_name)?;
        inner.write_i32(self.protocol_version)?;
        inner.write_i32(self.game_version)?;
        inner.write_i32(self.another_game_version)?;
        inner.write_string(&self.nickname)?;
        inner.write_bool(self.is_password)?;
        if self.is_password {
            inner.write_string(&self.password)?;
        };
        inner.write_string(&self.another_package_name)?;
        inner.write_string(&self.uuid_sum)?;
        inner.write_i32(self.client_units_checksum)?;
        inner.write_string(&self.token)?;
        inner.write_string(&self.color)?;

        let mut final_packet = Packet::new();
        let total_length = inner.payload.len() as i32 - 4;
        final_packet.write_i32(total_length)?;
        final_packet.write_bytes(&inner.payload)?;
        Ok(final_packet.payload)
    }
}