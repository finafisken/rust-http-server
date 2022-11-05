use rust_http_server::ThreadPool;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1338").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
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
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
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
