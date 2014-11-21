#![license = "MIT"]
#![feature(unboxed_closures, tuple_indexing, default_type_params, phase)]

//! Fast, safe middleware built on top of Hyper.

extern crate hyper;
extern crate modifier;
extern crate typemap;
extern crate plugin;
extern crate url;

#[cfg(test)]
#[phase(plugin)] extern crate stainless;

pub use request::Request;
pub use response::Response;

pub use hyper::status::StatusCode as Status;
pub use hyper::method::Method as Method;
pub use hyper::header::Headers;
pub use hyper::uri::RequestUri as RequestUri;

pub use hyper::net::{Fresh, Streaming};

use hyper::{HttpResult, net, server};
use std::io::net::ip::ToSocketAddr;
use std::fmt::Show;

pub use error::RailgunError;

pub mod request;
pub mod response;

mod impls;
mod error;

pub type RailgunResult<T> = Result<T, RailgunError>;

/// A Railgun Server.
///
/// Handles requests.
pub struct Railgun<F>
where F: Fn(Request, Response<net::Fresh>) + Send + Sync {
    handler: F
}

impl<F> Railgun<F>
where F: Fn(Request, Response<net::Fresh>) + Send + Sync {
    pub fn new(handler: F) -> Railgun<F> {
        Railgun { handler: handler }
    }

    pub fn handler(&self) -> &F {
        &self.handler
    }

    pub fn listen<T: ToSocketAddr + Show>(self, to: T) -> HttpResult<server::Listening> {
        let addr = to.to_socket_addr()
            .ok().expect(format!("Could not parse {} as a socket address.", to).as_slice());
        server::Server::http(addr.ip, addr.port).listen(F(self.handler))
    }
}

struct F<Fu: Fn(Request, Response<net::Fresh>) + Send + Sync>(Fu);

impl<Fu: Fn(Request, Response<net::Fresh>) + Send + Sync> server::Handler for F<Fu> {
    fn handle(&self, req: server::Request, res: server::Response<net::Fresh>) {
        let req = Request::lift(req);
        let res = Response::lift(res);

        self.0.call((req, res));
    }
}

