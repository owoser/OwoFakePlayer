use crate::packet::Packet;
use crate::error::PacketError;

// 错误类型增强版
#[derive(Debug)]
pub enum PacketError {
    OutOfBounds { requested: usize, available: usize },
    InvalidPacketType { expected: i32, actual: i32 },
    Utf8Error(std::string::FromUtf8Error),
    IoError(String),
}

impl fmt::Display for PacketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PacketError::OutOfBounds { requested, available } =>
                write!(f, "Read out of bounds (requested: {}, available: {})", requested, available),
            PacketError::InvalidPacketType { expected, actual } =>
                write!(f, "Invalid packet type (expected: {}, actual: {})", expected, actual),
            PacketError::Utf8Error(e) => write!(f, "UTF-8 error: {}", e),
            PacketError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

// Packet 结构体增强版
pub struct Packet {
    payload: Vec<u8>,
    offset: usize,
}

impl Packet {
    pub fn new() -> Self {
        Self {
            payload: Vec::new(),
            offset: 0,
        }
    }

    // 写入方法
    pub fn write_i32(&mut self, value: i32) {
        self.payload.extend_from_slice(&value.to_be_bytes());
    }

    pub fn write_string(&mut self, s: &str) {
        self.write_i32(s.len() as i32);
        self.payload.extend_from_slice(s.as_bytes());
    }

    // 读取方法
    pub fn read_i32(&mut self) -> Result<i32, PacketError> {
        self.read_bytes(4).map(|b| i32::from_be_bytes(b.try_into().unwrap()))
    }

    pub fn read_string(&mut self) -> Result<String, PacketError> {
        let len = self.read_i32()? as usize;
        let bytes = self.read_bytes(len)?;
        String::from_utf8(bytes).map_err(PacketError::Utf8Error)
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>, PacketError> {
        let available = self.payload.len() - self.offset;
        if len > available {
            return Err(PacketError::OutOfBounds {
                requested: len,
                available,
            });
        }

        let bytes = self.payload[self.offset..self.offset + len].to_vec();
        self.offset += len;
        Ok(bytes)
    }
}

// 注册连接包实现
pub const PACKET_REGISTER_CONNECTION: i32 = 160;

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
            protocol_version: 1,
            game_version: 173,
            another_game_version: 160,
            pkg_name: "com.corrodinggames.rts.java".to_string(),
            network_server_id: String::new(),
            server_key: 0,
            color: 0,
            zero: 0,
        }
    }

    // 序列化方法（关键修正点）
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut content = Packet::new();

        // 写入协议类型（包含在内容长度中）
        content.write_i32(PACKET_REGISTER_CONNECTION);

        // 写入其他字段
        content.write_string(&self.server_id);
        content.write_i32(self.protocol_version);
        content.write_i32(self.game_version);
        content.write_i32(self.another_game_version);
        content.write_string(&self.pkg_name);
        content.write_string(&self.network_server_id);
        content.write_i32(self.server_key);
        content.write_i32(self.color);
        content.write_i32(self.zero);

        // 构造完整数据包（长度头 + 内容）
        let mut final_packet = Packet::new();
        let content_length = content.payload.len() as i32 -4; // 这里已经包含协议类型的4字节
        final_packet.write_i32(content_length);
        final_packet.payload.extend(content.payload);

        final_packet.payload
    }

    // 反序列化方法（关键修正点）
    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        let mut packet = Packet {
            payload: data.to_vec(),
            offset: 0,
        };

        // 读取并验证长度头
        let declared_length = packet.read_i32()? as usize;
        if declared_length != packet.payload.len() - 8 {
            return Err(PacketError::OutOfBounds {
                requested: declared_length,
                available: packet.payload.len() - 8,
            });
        }

        // 读取协议类型
        let packet_type = packet.read_i32()?;
        if packet_type != PACKET_REGISTER_CONNECTION {
            return Err(PacketError::InvalidPacketType {
                expected: PACKET_REGISTER_CONNECTION,
                actual: packet_type,
            });
        }

        // 读取其他字段
        Ok(Self {
            server_id: packet.read_string()?,
            protocol_version: packet.read_i32()?,
            game_version: packet.read_i32()?,
            another_game_version: packet.read_i32()?,
            pkg_name: packet.read_string()?,
            network_server_id: packet.read_string()?,
            server_key: packet.read_i32()?,
            color: packet.read_i32()?,
            zero: packet.read_i32()?,
        })
    }
}

// 测试用例
fn main() -> Result<(), PacketError> {
    // 序列化测试
    let packet = RegisterConnectionPacket::new();
    let bytes = packet.to_bytes();
    println!("Serialized packet ({} bytes): {:?}", bytes.len(), bytes);

    // 反序列化测试
    let parsed = RegisterConnectionPacket::from_bytes(&bytes)?;

    println!("Parsed packet:");
    println!("Server ID: {}", parsed.server_id);
    println!("Protocol Version: {}", parsed.protocol_version);
    println!("Game Version: {}", parsed.game_version);
    println!("Another Game Version: {}", parsed.another_game_version);
    println!("Package Name: {}", parsed.pkg_name);
    println!("Network Server ID: {}", parsed.network_server_id);
    println!("Server Key: {}", parsed.server_key);
    println!("Color: {}", parsed.color);
    println!("Zero: {}", parsed.zero);

    Ok(())
}
