use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client {}", stream.peer_addr().unwrap());
                println!("Local addr {}", stream.local_addr().unwrap());

                handle_connection(stream);               
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let res = stream.read(&mut buffer);

    match res {
        Ok(res) => {
            println!("Read bytes: {}", res);
            println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
            
            let get = b"GET / HTTP/1.1\r\n";
            let (status_line, filename) = if buffer.starts_with(get) {        
                ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
            } else {
                ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
            };

            let content = fs::read_to_string(filename);
            match content {
                Ok(_) => {
                    println!("Read file");
                }
                Err(e) => {
                    println!("Err. File read: {}", e);
                    return;
                }
            }
            let response = format!("{}{}", status_line, content.unwrap());
            let r = stream.write(response.as_bytes());
            stream.flush().unwrap();
            match r {
                Ok(r) => {
                     println!("Write: {:?}" , r);
                }
                Err(e) => {
                    println!("Response err: {}", e);
                }
            }            
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}