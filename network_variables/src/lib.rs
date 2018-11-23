#![feature(try_from)]

//! # NetworkVariables
//! This libray aims to provide type used in serialization of
//! messages in the Minecraft network protocol ptite bite.

extern crate nom;

#[cfg(test)]
mod tests;

mod varint;
mod varlong;
mod network_string;

pub use self::varint::VarInt;
pub use self::varlong::VarLong;
pub use self::network_string::NetworkString;

