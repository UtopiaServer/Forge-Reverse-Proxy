
use super::{Packet, nom, parse_varint, parse_network_string};

use nom::be_u16;

named!(pub login<&[u8], Packet>,
    do_parse!(
        length: parse_varint >>
        id: parse_varint >>
        protocol: parse_varint >>
        host: parse_network_string >>
        port: be_u16 >>
        status: parse_varint >>
        (Packet::Log { length, id, protocol, host, port, status})
    )
);
