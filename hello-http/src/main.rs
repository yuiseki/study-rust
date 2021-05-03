use std::io::prelude::*;
use std::thread;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(tcp_stream: TcpStream) {
    let mut stream = std::io::BufReader::new(tcp_stream);
    let mut first_line = String::new();
    if let Err(err) = stream.read_line(&mut first_line) {
        panic!("error during receive a line: {}", err);
    }

    let mut params = first_line.split_whitespace();
    let _method = params.next();
    let path = params.next();

    match path {
        Some("/") => get_root(stream.get_mut()),
        Some("/hoge") => get_hoge(stream.get_mut()),
        Some("/fuga") => get_fuga(stream.get_mut()),
        _ => get_404(stream.get_mut())
    };
}

fn get_root(stream: &mut TcpStream) {
    let res = "HTTP/1.1 200 OK\r\n\r\nHello World!";
    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_hoge(stream: &mut TcpStream) {
    let res = "HTTP/1.1 200 OK\r\n\r\nhoge";
    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_fuga(stream: &mut TcpStream) {
    let res = "HTTP/1.1 200 OK\r\n\r\nfuga";
    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_404(stream: &mut TcpStream) {
    let res = "HTTP/1.1 404 OK\r\n\r\nNot Found";
    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}
