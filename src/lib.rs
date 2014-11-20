#![license = "MIT"]
#![feature(unboxed_closures, tuple_indexing)]

//! Fast, safe middleware built on top of Hyper.

extern crate hyper;
extern crate modifier;
extern crate typemap;
extern crate plugin;
extern crate url;

use request::Request;
use response::Response;

pub mod request;
pub mod response;

pub trait Handler {
    fn handle(&self, Request, Response);
}

pub struct Fun<F: Fn(Request, Response)>(F);

impl<F> Handler for Fun<F>
where F: Fn(Request, Response) {
    fn handle(&self, req: Request, res: Response) {
        self.0.call((req, res))
    }
}

/// A Railgun Server.
///
/// Handles requests.
pub struct Railgun<F>
where F: Handler {
    handler: F
}

impl<F> Railgun<F>
where F: Handler {
    pub fn new(handler: F) -> Railgun<F> {
        Railgun { handler: handler }
    }
}

