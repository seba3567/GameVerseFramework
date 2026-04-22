//! # Network Protocols
//!
//! Supported network protocols.

pub mod quic;
pub mod websocket;
pub mod http3;

pub use quic::QuicProtocol;
pub use websocket::WebSocketProtocol;
pub use http3::Http3Protocol;

/// Protocol enum for runtime polymorphism
#[derive(Debug, Clone)]
pub enum Protocol {
    Quic(QuicProtocol),
    WebSocket(WebSocketProtocol),
    Http3(Http3Protocol),
}
