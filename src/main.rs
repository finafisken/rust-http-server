use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // println!("Req: {:#?}", http_request);
    let http_header = "HTTP/1.1 200 OK";
    let body = fs::read_to_string("test.html").unwrap();
    let length = body.len();

    let response = format!("{http_header}\r\nContent-Length: {length}\r\n\r\n{body}");

    stream.write_all(response.as_bytes()).unwrap();
}
