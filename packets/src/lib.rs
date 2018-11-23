#[macro_use]
extern crate nom;

extern crate network_variables;
use network_variables::{VarInt, VarLong, NetworkString};

pub mod login;

// Enleve les warning chiant de si ou ca pas utilise.
#[allow(dead_code)]

/// All the differents packets that a Minecraft Client/Server can transfer.
#[derive(Debug, PartialEq, Eq)]
pub enum Packet {
    //Log(VarInt, VarInt, NetworkString<'a>, VarInt, VarInt),
    Log {
        length: VarInt,
        id: VarInt,
        protocol: VarInt,
        host: NetworkString,
        port: u16,
        status: VarInt
    },
    Others(VarInt)
}

fn parse_varint(bytes: &[u8]) -> nom::IResult<&[u8], VarInt> {
    VarInt::from_slice(bytes)
}

fn parse_network_string(bytes: &[u8]) -> nom::IResult<&[u8], NetworkString> {
    NetworkString::from_slice(bytes)
}
