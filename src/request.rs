use hyper::server;
use typemap::TypeMap;

pub struct Request {
    pub extensions: TypeMap,
}

impl Request {
    pub fn lift(_: server::Request) -> Request {
        Request {
            extensions: TypeMap::new()
        }
    }
}

