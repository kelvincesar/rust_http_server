use super::method::Methods;
pub struct Request {
    path: String,
    query: Option<String>,  // Permite que seja nulo durante inicialização. <> É usado para definir que é genérica over string.
    method: Methods,
}