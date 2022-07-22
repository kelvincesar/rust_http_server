use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {

        let stream = stream.unwrap();
        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).unwrap();
    println!("Conexão estabilizada!");
    
    // Closer look to an http request:
    //Method Request-URI HTTP-Version CRLF
    //headers CRLF
    //message-body
    

    println!(
        "- Request: {}",
        String::from_utf8_lossy(&buffer[..]) // buffer inteiro
    );

    
    let status_line = "HTTP/1.1 200 OK";

    let contents = fs::read_to_string("index.html").unwrap();
    // response: HTTP/1.1 200 OK\r\n\r\n
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    let status_line = "HTTP/1.1 200 OK";

    // evnia resposta
    stream.write(response.as_bytes()).unwrap();
    // will wait and prevent the program from continuing until 
    //jall the bytes are written to the connection
    stream.flush().unwrap(); 

}
