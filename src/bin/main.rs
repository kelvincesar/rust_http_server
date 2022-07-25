use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use http_server::ThreadPool;


fn main() {

    if let Ok(listener) = TcpListener::bind("localhost:7878") {
        let pool = ThreadPool::new(5);
        for stream in listener.incoming() {
            //println!("handling connection!");
            match stream {
                Ok(stream) => {
                    pool.execute(|| {
                        handle_connection(stream);
                    });
                },
                Err(error) => {
                    println!("Error when getting stream {:?}", error);
                }
            }
    
        }
    } else {
        println!("Deu erro!!! ");
    }
    


}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).unwrap();
    
    // Closer look to an http request:
    //Method Request-URI HTTP-Version CRLF
    //headers CRLF
    //message-body
    
    println!("Conexão estabilizada!");
    /*
    println!(
        "- Request: {}",
        String::from_utf8_lossy(&buffer[..]) // buffer inteiro
    );
    */
    
    let get = b"GET / HTTP/1.1\r\n"; // byte array
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // Direcionamento 
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    // response: HTTP/1.let status_line = "HTTP/1.1 200 OK";let status_line = "HTTP/1.1 200 OK";1 200 OK\r\n\r\n
   
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    // envia  resposta
    stream.write(response.as_bytes()).unwrap();
    // will wait and prevent the program from continuing until 
    //jall the bytes are written to the connection
    stream.flush().unwrap();     


    


    



}
