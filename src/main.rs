use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let response = match &request_line[..] {
        "GET / HTTP/1.1" => {
            let http_header = "200 OK";
            let body = fs::read_to_string("test.html").unwrap();
            build_response_string(http_header, &body)
        }
        _ => {
            let http_header = "404 Not Found";
            let body = fs::read_to_string("404.html").unwrap();
            build_response_string(http_header, &body)
        }
    };

    stream.write_all(response.as_bytes()).unwrap();
}

fn build_response_string(header: &str, body: &str) -> String {
    let length = body.len();
    format!("HTTP/1.1 {header}\r\nContent-Length: {length}\r\n\r\n{body}")
}
