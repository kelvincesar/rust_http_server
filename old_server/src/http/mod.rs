// Simplifica o import no arquivo main
pub use request::Request;
pub use method::Methods;

// Monta a arvore de arquivos para que possam ser importados no arquivo main para que seja possível importar individalmente os metodos
pub mod request;
pub mod method;

