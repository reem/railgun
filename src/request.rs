use hyper::server;
use typemap::TypeMap;

pub struct Request<'a> {
    pub extensions: TypeMap,
    hyper_request: server::Request<'a>
}

impl<'a> Request<'a> {
    pub fn lift(req: server::Request<'a>) -> Request<'a> {
        Request {
            extensions: TypeMap::new(),
            hyper_request: req
        }
    }
}

