// Módulo Server
// por padrão todos métodos são privados.
// por estar em um arquivo, não precisa declarar "mod..."

// https://doc.rust-lang.org/std/net/index.html
use std::net::TcpListener;
use std::io::Read;
use std::convert::TryFrom;
use std::convert::TryInto;
use crate::http::Request;

// Estrutura do servidor;
pub struct Server {
    addr: String,

}
// Implementaçao do servidor, similar à classe.
impl Server {
    // função associativa
    pub fn new(addr: String) -> Self {   // -> Self pode ser escrito -> Server
        println!("Inicializando servidor HTTP");
        // Retorno do servidor
        Server { addr }     // caso seja o mesmo nome, não precisa repetir addr: addr
    }

    // métodos
    // - &mut self para não adquirir os direitos da estrutura
    pub fn run(self) {
        println!("Escutando em {}", self.addr);

        // Escuta o endereço. unwrap usado para tratar erros de crash do software
        let listener = TcpListener::bind(&self.addr).unwrap();

        // Mesma coisa que while true
        // Label para o loop, permitindo realizar break ou continue em caso de inner loops.
        'network:loop {
            // Bloqueia a thread até uma nova conexão seja realizada.
            // Retorna uma tuple. 
            match listener.accept() {
                Ok((mut stream, addr)) => { // Pode-se usar "_" para dizer que deseja ignorar o retorno
                    println!("Conexão aceita! Endereço: {}", addr);

                    // Cria array com 1024 posições preeenchidas com zero
                    let mut buffer = [0; 1024]; 

                    // Leitura do socket
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recebido: {}", String::from_utf8_lossy(&buffer));
                            
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Erro ao realizar parse da requisição: {}", e)
                            }
                        
                        }
                        Err(e) => println!("Erro ao ler as mensagens da conexão {}", e),
                    }
                }
                Err(e) => println!("Erro ao aceitar conexão: {}", e),
                // pode-se usar "_" para case default
            }
        }

    }
}