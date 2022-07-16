use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::thread;
use std::time::Duration;

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // take 2 requests then shut down
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
             handle_connection(stream);
         })
    }


    println!("Shut down.")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    // If this line is here the content of buffer will be empty because it has been read out
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get =b"GET /get HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
        
    }else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    
    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}