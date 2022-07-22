// Importa os módulos
use http::Request;
use http::Methods;
use server::Server;

mod server;
mod http;



// usar echo "teste" | netcat localhost 8080 para debugar

fn main() {
    // Iniciliza o servidor
    let server = Server::new("localhost:8080".to_string());
    server.run();
}

