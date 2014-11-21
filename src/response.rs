use hyper::{server, net};
use typemap::TypeMap;

pub struct Response<'a, W> {
    pub extensions: TypeMap,
    hyper_res: server::Response<'a, W>
}

impl<'a> Response<'a, net::Fresh> {
    pub fn lift(res: server::Response<'a, net::Fresh>) -> Response<'a, net::Fresh> {
        Response {
            extensions: TypeMap::new(),
            hyper_res: res
        }
    }
}

