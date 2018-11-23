extern crate packets;

use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::{Read};

fn handle_client(stream: &mut TcpStream) {
    let mut buf = vec![];
    loop {
        stream.read_to_end(&mut buf).expect("IO error");
        break;
    };
    while buf.len() > 0 {
        let result =  packets::parse_packet(buf.as_slice());
        if let Ok(date) = result {
            println!("packet: {:?}", date.1);
            buf = date.0.to_vec();
        } else if let Err(a) = result { // Taimerais faire un truc genre OOOOOOOOh je sais pas si sa marche 
            eprintln!("Error.... {:?}", a);
        }
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2048")?;
    for stream in listener.incoming() {
        handle_client(&mut stream?);
    }
    Ok(())
}
