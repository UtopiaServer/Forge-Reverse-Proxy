use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::{Read};

fn handle_client(stream: &mut TcpStream) {
    let mut buf = vec![];
    loop {
        match stream.read_to_end(&mut buf) {
            Ok(_) => break,
        Err(e) => panic!("encountered IO error: {}", e),
        }
    };
    println!("bytes: {:?}", String::from_utf8(buf));
}



pub fn main() -> io::Result<()> {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:2048")?;

    for stream in listener.incoming() {
        handle_client(&mut stream?);
    }
    Ok(())
}