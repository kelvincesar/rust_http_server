use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        //println!("handling connection!");
        let stream = stream.unwrap();
        handle_connection(stream);
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
    println!(
        "- Request: {}",
        String::from_utf8_lossy(&buffer[..]) // buffer inteiro
    );

    
    let get = b"GET / HTTP/1.1\r\n"; // byte array

    // Direcionamento 
    let (status_line, file_name) = 
        if buffer.starts_with(get){
            ("HTTP/1.1 200 OK", "index.html")

        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    let contents = fs::read_to_string(file_name).unwrap();
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
