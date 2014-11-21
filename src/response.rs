use std::io::IoResult;

use hyper::server;
use typemap::TypeMap;

use {RailgunResult, Fresh, Streaming};

pub struct Response<'a, W = Fresh> {
    pub extensions: TypeMap,
    hyper_res: server::Response<'a, W>
}

impl<'a> Response<'a, Fresh> {
    pub fn lift(res: server::Response<'a, Fresh>) -> Response<'a, Fresh> {
        Response {
            extensions: TypeMap::new(),
            hyper_res: res
        }
    }

    pub fn start(self) -> RailgunResult<Response<'a, Streaming>> {
        Ok(Response {
            extensions: self.extensions,
            hyper_res: try!(self.hyper_res.start())
        })
    }
}

impl<'a> Response<'a, Streaming> {
    pub fn end(self) -> RailgunResult<()> {
        try!(self.hyper_res.end());
        Ok(())
    }

}

impl<'a> Writer for Response<'a, Streaming> {
    fn write(&mut self, buf: &[u8]) -> IoResult<()> {
        self.hyper_res.write(buf)
    }
}

