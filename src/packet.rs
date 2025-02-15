use std::convert::TryInto;
use crate::error::PacketError;

pub struct Packet {
    pub payload: Vec<u8>,
    pub offset: usize,
}

impl Packet {
    pub fn new() -> Self {
        Self {
            payload: Vec::new(),
            offset: 0,
        }
    }

    pub fn write_i16(&mut self, value: i16) -> Result<(), PacketError> {
        self.payload.extend_from_slice(&value.to_be_bytes());
        Ok(())
    }

    pub fn write_i32(&mut self, value: i32) -> Result<(), PacketError> {
        self.payload.extend_from_slice(&value.to_be_bytes());
        Ok(())
    }
    pub fn write_i64(&mut self, value: i64) -> Result<(), PacketError> {
        self.payload.extend_from_slice(&value.to_be_bytes());
        Ok(())
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), PacketError> {
        self.payload.extend_from_slice(bytes);
        Ok(())
    }
    pub fn write_byte(&mut self, value: u8) -> Result<(), PacketError> {
        self.payload.push(value);
        Ok(())
    }


    pub fn write_string(&mut self, s: &str) -> Result<(), PacketError> {
        let bytes = s.as_bytes();
        self.write_i16(bytes.len() as i16)?;
        self.write_bytes(bytes)?;
        Ok(())
    }

    pub fn write_is_string(&mut self, s: &str) -> Result<(), PacketError> {
        if s.is_empty() {
            self.write_bool(false)?;
            return Ok(());
        }

        self.write_bool(true)?;
        self.write_i16(s.len() as i16)?;
        self.write_bytes(s.as_bytes())?;
        Ok(())
    }

    pub fn write_bool(&mut self, value: bool) -> Result<(), PacketError> {
        self.payload.push(value as u8);
        Ok(())
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>, PacketError> {
        let end = match self.offset.checked_add(len) {
            Some(e) => e,
            None => return Err(PacketError::OutOfBounds),
        };
        if end > self.payload.len() {
            return Err(PacketError::OutOfBounds);
        }
        let bytes = self.payload[self.offset..end].to_vec();
        self.offset = end;
        Ok(bytes)
    }

    pub fn read_bool(&mut self) -> Result<bool, PacketError> {
        Ok(self.read_bytes(1)?[0] != 0)
    }

    pub fn read_byte(&mut self) -> Result<u8, PacketError> {
        Ok(self.read_bytes(1)?[0])
    }

    pub fn read_i16(&mut self) -> Result<i16, PacketError> {
        let bytes = self.read_bytes(2)?;
        Ok(i16::from_be_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_i32(&mut self) -> Result<i32, PacketError> {
        let bytes = self.read_bytes(4)?;
        Ok(i32::from_be_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_i64(&mut self) -> Result<i64, PacketError> {
        let bytes = self.read_bytes(8)?;
        Ok(i64::from_be_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_string(&mut self) -> Result<String, PacketError> {
        let len = self.read_i16()? as usize;
        let bytes = self.read_bytes(len)?;
        String::from_utf8(bytes).map_err(Into::into)
    }

    pub fn read_is_string(&mut self) -> Result<String, PacketError> {
        if !self.read_bool()? {
            return Ok(String::new());
        }
        let len = self.read_i16()? as usize;
        let bytes = self.read_bytes(len)?;
        String::from_utf8(bytes).map_err(Into::into)
    }
}