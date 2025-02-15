use chrono::{Local, Utc};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::error::PacketError;
use crate::packet::Packet;
use crate::protocol::heart::HeartPacket;
use crate::protocol::heart_beat::HeartBeatPacket;
use crate::protocol::player_info::PlayerInfoPacket;
use crate::protocol::preregister_connection::PreregisterConnectionPacket;
use crate::protocol::register_connection::RegisterConnectionPacket;

#[derive(Debug, PartialEq)]
pub struct PacketModel {
    pub model:i32,
    pub total_length: i32,
}
pub trait ToBytes {
    fn to_bytes(&self) -> Result<Vec<u8>, PacketError>;
}
pub trait FromBytes :Sized {
    fn from_packet(packet: &mut Packet) -> Result<Self, PacketError>;
}
pub fn make_packet<T: ToBytes>(packet: &T) -> Vec<u8> {
    let mut packet1 = packet.to_bytes().unwrap();
    packet1
}
impl FromBytes for PacketModel {
    fn from_packet(packet: &mut Packet) -> Result<Self, PacketError> {
        let total_length = packet.read_i32()?;
        let model = packet.read_i32()?;;
        Ok(Self { total_length,model })
    }
}
pub async fn packet_con(a:&[u8], stream: &mut TcpStream) -> Result<(), PacketError> {
    let mut packet1 = Packet{
        payload: Vec::from(a),
        offset: 0,
    };
    let packet_type = PacketModel::from_packet(&mut packet1)?;
    packet1.offset = 0;
    match packet_type.model {
         161 =>{
             println!("收到161数据包 正在发送注册包");
             let mut packet = PlayerInfoPacket::new(&mut packet1);
             println!("{:?}",packet);
             send_packet(stream, &mut packet).await.expect("");
         }
         108 =>{
            let b = HeartPacket::from_packet(&mut packet1)?;
            let mut packet = HeartBeatPacket::new(b.ping_number);
             println!("{:?}",packet);
            send_packet(stream, &mut packet).await.expect("");
         }

        _ => {
        }
    }
    Ok(())
}
pub async fn send_packet<T: ToBytes>(stream :&mut TcpStream, packet: &T) -> Result<(), Box<dyn std::error::Error>> {
    let msg = make_packet(packet);
    stream.write_all(&msg).await?;
    Ok(())
}


