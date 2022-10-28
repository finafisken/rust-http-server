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
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // println!("Req: {:#?}", http_request);

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let response = match &request_line[..] {
        "GET / HTTP/1.1" => {
            let http_header = "HTTP/1.1 200 OK";
            let body = fs::read_to_string("test.html").unwrap();
            build_response_string(http_header, &body)
        }
        _ => {
            let http_header = "HTTP/1.1 404 NOT_FOUND";
            build_response_string(http_header, "")
        }
    };
    // let http_header = "HTTP/1.1 200 OK";
    // let body = fs::read_to_string("test.html").unwrap();
    // let response = build_response_string(http_header, &body);

    stream.write_all(response.as_bytes()).unwrap();
}

fn build_response_string(header: &str, body: &str) -> String {
    let length = body.len();
    format!("{header}\r\nContent-Length: {length}\r\n\r\n{body}")
}
