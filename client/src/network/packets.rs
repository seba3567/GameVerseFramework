//! # Packet Serialization
//!
//! FlatBuffers-based packet serialization.

use flatbuffers::{FlatBufferBuilder, WIPOffset};
use serde::{Serialize, Deserialize};

/// Packet types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PacketType {
    Hello,
    Accepted,
    Rejected,
    Event,
    RPC,
    ResourceList,
    ResourceData,
    Ping,
    Pong,
}

/// Network packet
#[derive(Debug, Clone)]
pub struct Packet {
    pub packet_type: PacketType,
    pub data: Vec<u8>,
}

impl Packet {
    /// Create a new packet
    pub fn new(packet_type: PacketType, data: Vec<u8>) -> Self {
        Self {
            packet_type,
            data,
        }
    }
    
    /// Serialize packet to bytes
    pub fn serialize(&self) -> Vec<u8> {
        // Simple length-prefixed serialization
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(self.data.len() as u32).to_le_bytes());
        bytes.push(self.packet_type as u8);
        bytes.extend_from_slice(&self.data);
        bytes
    }
    
    /// Deserialize packet from bytes
    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 5 {
            return None;
        }
        
        let len = u32::from_le_bytes(bytes[0..4].try_into().ok()?) as usize;
        let packet_type = bytes[4];
        let data = bytes[5..5 + len].to_vec();
        
        Some(Packet {
            packet_type: match packet_type {
                0 => PacketType::Hello,
                1 => PacketType::Accepted,
                2 => PacketType::Rejected,
                3 => PacketType::Event,
                4 => PacketType::RPC,
                5 => PacketType::ResourceList,
                6 => PacketType::ResourceData,
                7 => PacketType::Ping,
                8 => PacketType::Pong,
                _ => return None,
            },
            data,
        })
    }
}

/// Packet builder for FlatBuffers
pub struct PacketBuilder<'a> {
    fbb: FlatBufferBuilder<'a>,
}

impl<'a> PacketBuilder<'a> {
    /// Create a new packet builder
    pub fn new() -> Self {
        Self {
            fbb: FlatBufferBuilder::new(),
        }
    }
    
    /// Build HELLO packet
    pub fn build_hello(&mut self, version: &str, auth_token: Option<&str>) -> WIPOffset<u8> {
        // Placeholder for FlatBuffers serialization
        self.fbb.create_vector(&[])
    }
}
