use hello::ThreadPool;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut pool = ThreadPool::new(2);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            let handle = thread::current();
            let id = handle.id();
            println!("Starting job {:?}", id);
            handle_connection(stream);
            println!("Finished job {:?}", id);
        });
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = "GET / HTTP/1.1";
    let sleep = "GET /sleep HTTP/1.1";

    let request_line = buffer.lines().next().unwrap().unwrap();
    let (status, filename) = if &request_line == get {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if &request_line == sleep {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let len = contents.len();
    let response = format!("{status}\r\nContent-Length: {len}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
