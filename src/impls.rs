use {Request, Response};

include!(concat!(env!("OUT_DIR"), "/impls.rs"))

impl<F> Fn(Request, Response) for (F,)
where F: Fn(Request, Response) {
    extern "rust-call" fn call(&self, (req, res): (Request, Response)) {
        self.0.call((req, res))
    }
}

impl<F1, F2, T1> Fn(Request, Response) for (F1, F2)
where F1: Fn(Request, Response) -> T1,
      F2: Fn(T1) {
    extern "rust-call" fn call(&self, conn: (Request, Response)) {
        let a = self.0.call(conn);
        let a = self.1.call((a,));
        return a;
    }
}

