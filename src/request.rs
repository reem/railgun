use hyper::server;
use typemap::TypeMap;

pub struct Request {
    pub extensions: TypeMap,
    hyper_request: server::Request
}

