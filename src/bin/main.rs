use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server started...");

    let thread_pool_size = 4;
    println!("Creating thread pool with {} workers...", thread_pool_size);
    let mut pool = ThreadPool::new(thread_pool_size);

    //for stream in listener.incoming().take(2) {
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    //println!("\nRequest: {}\n", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (filename, status_code) = if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("static/slow.html", "200 OK")
    } else if buffer.starts_with(get) {
        ("static/hello.html", "200 OK")
    } else {
        ("static/404.html", "404 NOT FOUND")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}", status_code, contents.len(), contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    //println!("\nResponse: {}\n", String::from_utf8_lossy(&response.as_bytes()));
}