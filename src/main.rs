use std::fs;
use std::io::{BufRead, BufReader, Result, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> Result<()> {
    let addr = "127.0.0.1:7878";
    TcpListener::bind(addr)
        .unwrap()
        .incoming()
        .map(|stream| stream.unwrap())
        .for_each(handle_connection);

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let (status_line, filename) = BufReader::new(&stream)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .eq("GET / HTTP/1.1")
        .then_some(("HTTP/1.1 200 OK", "./public/index.html"))
        .unwrap_or(("HTTP/1.1 404 NOT FOUND", "./public/404.html"));

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
}
