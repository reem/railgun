use std::io::IoResult;

use hyper::server;
use typemap::TypeMap;

use {Method, Headers, RequestUri};

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

    pub fn method(&self) -> Method {
        self.hyper_request.method.clone()
    }

    pub fn headers(&self) -> &Headers {
        &self.hyper_request.headers
    }

    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.hyper_request.headers
    }

    pub fn uri(&self) -> &RequestUri {
        &self.hyper_request.uri
    }
}

impl<'a> Reader for Request<'a> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
        self.hyper_request.read(buf)
    }
}

