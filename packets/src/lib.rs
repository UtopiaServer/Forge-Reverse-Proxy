#[macro_use]
extern crate nom;

extern crate network_variables;
use network_variables::{VarInt, VarLong, NetworkString};

pub mod login;

// Enleve les warning chiant de si ou ca pas utilise.
#[allow(dead_code)]

#[derive(Debug, PartialEq, Eq)]
pub struct Packet {
    length: VarInt,
    id: VarInt,
    data: PacketData
}

/// All the differents packets that a Minecraft Client/Server can transfer.
#[derive(Debug, PartialEq, Eq)]
pub enum PacketData {
    Handshake {
        protocol: VarInt,
        host: NetworkString,
        port: u16,
        status: VarInt
    },
    Login {
        username: NetworkString
    },
    Status
}

fn parse_varint(bytes: &[u8]) -> nom::IResult<&[u8], VarInt> {
    VarInt::from_slice(bytes)
}

fn parse_network_string(bytes: &[u8]) -> nom::IResult<&[u8], NetworkString> {
    NetworkString::from_slice(bytes)
}

fn log(bytes: &[u8]) -> nom::IResult<&[u8], ()> {
    println!("{:?}", bytes);
    Ok((bytes, ()))
}

named!(pub parse_packet<&[u8], Packet>, do_parse!(
    length: parse_varint >>
    id: peek!(parse_varint) >>
    data: switch!(parse_varint,
        VarInt(0) => alt!(
            complete!(call!(login::handshake)) |
            complete!(call!(login::login_start)) |
            call!(login::status)
        )
    ) >>
    ( Packet {length, id, data } )
));
