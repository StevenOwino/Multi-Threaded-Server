// This code will listen  for an HTTP request at the address 127.0.0.1:3571 for incoming
// Tcp streams, respond, and render the HTML source file, ASYNCHRONOUSLY.
// The "take" method can be defined in the Iterator trait to limit the iteration
// to the first two items at most. 
// The ThreadPool will go out of scope at the end of main,
// and the "drop" implementation will run. 

use hello::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:3571").unwrap();
    // Ports 0 to 1023 are Well-Known Ports.
    // Ports 1024 to 49151 are Registered Ports (often registered by a software developer 
    // to designate a particular port for their application)
    // Ports 49152 to 65535 are Public Ports.

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {  
    // insert .take(2) before the opening curly brace above
    // To demonstrate graceful shutdown and cleanup
        let stream = stream.unwrap();
        
        pool.execute(|| {   
        //thread::spawn(|| {
            handle_connection(stream);
        });

    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b" /sleep HTTP/1.1\r\n";


    //if buffer.starts_with(get) {

    //  let contents = fs::read_to_string("hello.html").unwrap();

    //  let response = format!( 
    //      "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    //      contents.len(),
    //      contents
    //);

    //stream.write(response.as_bytes()).unwrap();
    //stream.flush().unwrap();

  //}

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!( 
          "{}\r\nContent-Length: {}\r\n\r\n{}",
          status_line,
          contents.len(),
          contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();


    
  }

    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

// Remember to stop the program by pressing ctrl-c 
// when you’re done running a particular version of the code. 
// Then restart: cargo run 'hello' after you’ve made each set of code changes
// to make sure you’re running the newest code.
