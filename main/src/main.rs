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
    println!("{:?}", buf);
    println!("bytes: {:?}", packets::login::login(buf.as_slice()));
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2048")?;
    for stream in listener.incoming() {
        handle_client(&mut stream?);
    }
    Ok(())
}
