use hyper::{server, net};
use typemap::TypeMap;

pub struct Response {
    pub extensions: TypeMap,
}

impl Response {
    pub fn lift(_: server::Response<net::Fresh>) -> Response {
        Response {
            extensions: TypeMap::new()
        }
    }
}

