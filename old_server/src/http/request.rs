use super::method::Methods;
use std::convert::TryFrom; //https://doc.rust-lang.org/std/convert/index.html

pub struct Request {
    path: String,
    query: Option<String>,  // Permite que seja nulo durante inicialização. <> É usado para definir que é genérica over string.
    method: Methods,
}





impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}