//! Rusty_Channel
//!
//! A library for sending data unidirectionally

pub mod packets;
pub mod reciever;
pub mod transmitter;
pub use reciever::UdpReciever;
pub use transmitter::UdpTransmitter;
