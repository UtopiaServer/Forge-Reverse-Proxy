
use super::{PacketData, nom, parse_varint, parse_network_string};

use nom::be_u16;

named!(pub handshake<&[u8], PacketData>,
    do_parse!(
        protocol: parse_varint >>
        host: parse_network_string >>
        port: be_u16 >>
        status: parse_varint >>
        (PacketData::Handshake {protocol, host, port, status})
    )
);

named!(pub login_start<&[u8], PacketData>,
    do_parse!(
        username: parse_network_string >>
        (PacketData::Login {username})
    )
);

named!(pub status<&[u8], PacketData>,
    do_parse!(
        (PacketData::Status)
    )
);
