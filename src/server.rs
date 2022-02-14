// Módulo Server
// por padrão todos métodos são privados.
// por estar em um arquivo, não precisa declarar "mod..."


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
        Server { 
            addr  // caso seja o mesmo nome, não precisa repetir addr: addr
        }
    }

    // métodos
    // - &mut self para não adquirir os direitos da estrutura
    pub fn run(self) {
        println!("Escutando em {}", self.addr);

    }
}