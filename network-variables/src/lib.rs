#![feature(try_from)]

//! # NetworkVariables
//! This libray aims to provide type used in serialization of
//! messages in the Minecraft network protocol

#[cfg(test)]
mod tests;

mod varint;
mod varlong;

pub use self::varint::VarInt;
pub use self::varlong::VarLong;

#[derive(Debug, PartialEq, Eq)]
pub enum VarErr {
    TooBig,
    NoBytes
}

