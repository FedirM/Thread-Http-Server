use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

mod cli;
mod default_404;

use cli::parse_cli;
use default_404::not_found_page;

fn main() {
    let (port, host_dir) = parse_cli();
    let addr = format!("127.0.0.1:{port}");

    let server = TcpListener::bind(&addr).expect("Could not start server!");
    println!("Started on {}...", &addr);
    println!("Served directory: {}", host_dir.to_string_lossy());

    for stream in server.incoming() {
        let stream = stream.expect("Could not get stream!");
        handle_connect(stream);
    }
}

fn handle_connect(mut stream: TcpStream) {
    let mut buf = [0_u8; 1024];

    stream.read(&mut buf).expect("Could not read from stream!");

    println!("Req: {}", String::from_utf8_lossy(&buf));

    let first_line_header = std::str::from_utf8(&buf).unwrap().lines().nth(0).unwrap();
    let split_res: Vec<&str> = first_line_header.split(' ').collect();
    println!("SPLIT: {:#?}", split_res);

    let response = if split_res.len() > 1 && split_res[0] == "GET" {
        match split_res[1] {
            "/" => {
                let 
            }
        }
    } else {
        let nfp = not_found_page();
        String::from(format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\n\r\n\r{}",
            nfp.len(),
            nfp
        ))
    };

    stream
        .write(response.as_bytes())
        .expect("Could not send response");

    stream.flush().expect("Cannot flush the stream!");
}
