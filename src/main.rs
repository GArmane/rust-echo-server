use std::net::{TcpListener, TcpStream};
use std::io::Result;

fn main() -> Result<()> {
    let addr = "127.0.0.1:7878";
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        stream.unwrap();
        println!("Connection established");
    }

    Ok(())
}

