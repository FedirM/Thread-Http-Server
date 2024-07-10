use std::fs::read_to_string;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

mod cli;
mod default_404;
mod thread_pool;

use cli::parse_cli;
use default_404::not_found_page;
use thread_pool::ThreadPool;

fn main() {
    let (port, host_dir) = parse_cli();
    let addr = format!("127.0.0.1:{port}");

    let server = TcpListener::bind(&addr).expect("Could not start server!");
    println!("Started on {}...", &addr);
    println!("Served directory: {}", host_dir.to_string_lossy());

    let th_pool = ThreadPool::new();

    for stream in server.incoming().take(2) {
        let stream: TcpStream = stream.expect("Could not get stream!");
        let search_dir = host_dir.clone();

        th_pool.exec(|| handle_connect(stream, search_dir));
    }
}

fn handle_connect(mut stream: impl Read + Write, mut host_dir: PathBuf) {
    let mut buf = [0_u8; 1024];

    stream.read(&mut buf).expect("Could not read from stream!");

    let first_line_header = std::str::from_utf8(&buf).unwrap().lines().nth(0).unwrap();
    let split_res: Vec<&str> = first_line_header.split(' ').collect();
    println!("SPLIT: {:#?}", split_res);

    let response = if split_res.len() > 1 && split_res[0] == "GET" {
        if split_res[1] == "/" {
            host_dir.push("index.html");
        } else {
            println!("Before push: {} ", host_dir.to_str().unwrap());
            host_dir.push(&split_res[1][1..]);
            println!("After push: {} ", host_dir.to_str().unwrap());
        }

        try_to_read(&host_dir)
    } else {
        let nfp = not_found_page();
        String::from(format!(
            "HTTP/1.1 404 Not Found\r\nContent-Length: {}\n\r\n\r{}",
            nfp.len(),
            nfp
        ))
    };

    println!("RESPONSE: {}", response);

    stream
        .write(response.as_bytes())
        .expect("Could not send response");

    stream.flush().expect("Cannot flush the stream!");
}

fn try_to_read(filename: &PathBuf) -> String {
    println!("Try to open file: {}", filename.to_str().unwrap());
    return match read_to_string(&filename) {
        Ok(content) => String::from(format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\n\r\n\r{}",
            content.len(),
            content
        )),
        Err(_) => {
            let nfp = not_found_page();
            String::from(format!(
                "HTTP/1.1 404 Not Found\r\nContent-Length: {}\n\r\n\r{}",
                nfp.len(),
                nfp
            ))
        }
    };
}
