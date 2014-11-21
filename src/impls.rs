use hyper::net::Fresh;
use {Request, Response};

include!(concat!(env!("OUT_DIR"), "/impls.rs"))

#[cfg(test)]
mod test {
    pub use {Request, Response};

    describe! impls {
        it "should impl Fn for chains of Fn that start with Fn(Request, Response)" {
            use std::mem;

            let chain = (
                |&: _: Request, _: Response| { 8u },
                |&: x: uint| { (x + 8).to_string() },
                |&: x: String| { x.len() },
                |&: x: uint| { x }
            );

            unsafe {
                let req: Request = mem::uninitialized();
                let res: Response = mem::uninitialized();
                chain.call((req, res));
            }
        }
    }
}


