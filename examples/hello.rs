#![feature(unboxed_closures)]
extern crate railgun;

use railgun::{Railgun, Request, Response};

fn main() {
    Railgun::new(|&: _: Request, res: Response| {
        let mut res = res.start().unwrap();
        res.write(b"Hello World!").unwrap();
        res.end().unwrap();
    }).listen("localhost:3000").ok().expect("Could not start server.");
}

