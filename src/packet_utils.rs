use std::collections::hash_map::Keys;
use std::fmt::Write;
use num_bigint::BigInt;
use sha2::{Digest, Sha256};
use uuid::Uuid;
use crate::packet::Packet;
use crate::protocol::register_connection::RegisterConnectionPacket;
pub struct SerKey {
    pub keys: i32,
    pub network_id: String,
    pub color :i32
}
impl SerKey {
    pub fn new() -> Self {
        Self {
            keys: 0,
            network_id: String::new(),
            color: 0,
        }
    }
    pub fn get(a: &mut SerKey, packet: &mut Packet) {
        let b = RegisterConnectionPacket::from_packet(packet).unwrap();
        a.keys = b.server_key;
        a.network_id = b.network_server_id;
        a.color = b.color;
    }
}
fn format_scientific(n: &BigInt) -> String {
    let s = n.to_string();
    if s == "0" {
        return "0E0".to_string();
    }
    let mut chars: Vec<char> = s.chars().collect();
    let mut sign = "";
    if chars[0] == '-' {
        sign = "-";
        chars = chars[1..].to_vec();
    }
    if chars.is_empty() {
        return "0E0".to_string();
    }
    let len = chars.len();
    let exponent = len - 1;
    if len > 1 {
        chars.insert(1, '.');
    }
    let mut mantissa: String = chars.into_iter().collect();
    mantissa = mantissa.trim_end_matches('0').to_string();
    mantissa = mantissa.trim_end_matches('.').to_string();
    if mantissa.is_empty() {
        mantissa = "0".to_string();
    }
    format!("{}{}E{}", sign, mantissa, exponent)
}
pub fn compute_key_for_packet(server_key: i32) -> String {
    let server_key_bi = BigInt::from(server_key);
    let t1_ratio = BigInt::from(44000);
    let t1 = t1_ratio * &server_key_bi;
    let t1_str = format_scientific(&t1);
    fn get_ratios(num: i32) -> i32 {
        match num {
            0 => 4000,
            1 => 0,
            2 => 1000,
            3 => 2000,
            4 => 5000,
            5 => 10000,
            6 => 50000,
            7 => 100000,
            8 => 200000,
            _ => 999,
        }
    }

    format!(
        "c:{}m:{}0:{}1:{}2:{}3:{}4:{}5:{}6:{}7:{}8:{}t1:{}d:{}",
        server_key,
        server_key * 87 + 24,
        get_ratios(0).wrapping_mul(11).wrapping_mul(server_key),
        get_ratios(1).wrapping_mul(12).wrapping_add(server_key),
        get_ratios(2).wrapping_mul(13).wrapping_mul(server_key),
        get_ratios(3).wrapping_mul(14).wrapping_add(server_key),
        get_ratios(4).wrapping_mul(15).wrapping_mul(server_key),
        get_ratios(5).wrapping_mul(16).wrapping_add(server_key),
        get_ratios(6).wrapping_mul(17).wrapping_mul(server_key),
        get_ratios(7).wrapping_mul(18).wrapping_mul(server_key),
        get_ratios(8).wrapping_mul(19).wrapping_mul(server_key),
        t1_str,
        5 * server_key
    )
}

pub fn compute_color_for_packet(color: i32) -> String {
    format!("#{:06X}", color & 0x00FFFFFF)
}
pub fn create_packet(packet_type: u32, payload: &[u8]) -> Vec<u8> {
    let type_bytes = packet_type.to_be_bytes();
    let length = (payload.len() as u32).to_be_bytes();
    let mut result = Vec::with_capacity(8 + payload.len());
    result.extend_from_slice(&length);
    result.extend_from_slice(&type_bytes);
    result.extend_from_slice(payload);
    result
}
pub fn compute_sha256_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = String::with_capacity(result.len() * 2);
    for byte in result {
        write!(hash, "{:02X}", byte).unwrap();
    }
    hash
}

fn uuid_to_csharp_guid_bytes(uuid: Uuid) -> [u8; 16] {
    let bytes = uuid.as_bytes();
    let mut csharp_bytes = [0u8; 16];
    // 调整前4字节为小端序
    csharp_bytes[0..4].copy_from_slice(&bytes[0..4].iter().rev().cloned().collect::<Vec<_>>());
    // 调整接下来的2字节为小端序
    csharp_bytes[4..6].copy_from_slice(&bytes[4..6].iter().rev().cloned().collect::<Vec<_>>());
    // 调整接下来的2字节为小端序
    csharp_bytes[6..8].copy_from_slice(&bytes[6..8].iter().rev().cloned().collect::<Vec<_>>());
    // 剩余字节保持不变
    csharp_bytes[8..16].copy_from_slice(&bytes[8..16]);
    csharp_bytes
}

pub fn compute_uuid_for_packet(client_uuid: &str, server_uuid: &str) -> String {
    let client_guid = Uuid::parse_str(client_uuid).expect("Invalid client UUID");
    let server_guid = Uuid::parse_str(server_uuid).expect("Invalid server UUID");

    let client_bytes = uuid_to_csharp_guid_bytes(client_guid);
    let server_bytes = uuid_to_csharp_guid_bytes(server_guid);

    let client_num = BigInt::from_signed_bytes_le(&client_bytes);
    let server_num = BigInt::from_signed_bytes_le(&server_bytes);

    let sum_guid = client_num + server_num;
    let sum_bytes = sum_guid.to_signed_bytes_le();

    compute_sha256_hash(&sum_bytes)
}