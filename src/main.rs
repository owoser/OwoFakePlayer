mod network;
mod protocol;
mod packet;
mod error;
mod packet_utils;

use std::{io, net};
use std::error::Error;
use std::net::IpAddr;
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::network::{make_packet, packet_con, send_packet, ToBytes};
use crate::protocol::heart_beat::HeartBeatPacket;
use crate::protocol::preregister_connection::PreregisterConnectionPacket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut stream = TcpStream::connect("192.168.1.7:5123").await?;
    println!("正在连接服务器:{:?}", stream);

    // 发送消息 预注册包
    let mut packet = PreregisterConnectionPacket::new();
    send_packet(&mut stream, &mut packet).await?;

    // 读取响应
    let mut buf = [0; 1024];
    loop{
        match stream.read(&mut buf).await{
            Ok(0) => {
                println!("连接已经关闭");
                break;
            }
            Ok(n) => {
                let mut a = &buf[..n];
                packet_con(a, &mut stream).await.expect("TODO: panic message");
            }
            Err(e) => {
                eprintln!("读取错误:{}",e);
                break;
            }
        }
    }
    Ok(())
}