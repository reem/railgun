#![license = "MIT"]
#![feature(unboxed_closures, tuple_indexing)]

//! Fast, safe middleware built on top of Hyper.

extern crate hyper;
extern crate modifier;
extern crate typemap;
extern crate plugin;
extern crate url;

pub use request::Request;
pub use response::Response;

pub mod request;
pub mod response;

mod impls;

/// A Railgun Server.
///
/// Handles requests.
pub struct Railgun<F>
where F: Fn(Request, Response) {
    handler: F
}

impl<F> Railgun<F>
where F: Fn(Request, Response) {
    pub fn new(handler: F) -> Railgun<F> {
        Railgun { handler: handler }
    }

    pub fn handler(&self) -> &F {
        &self.handler
    }
}

