use std::os;
use std::io::File;

fn main() {
    let dst = Path::new(os::getenv("OUT_DIR").expect("build script run outside of Cargo"));
    let f = File::create(&dst.join("impls.rs")).ok().expect("could not create out file");

    generate(f, 32);
}

fn generate(mut file: File, limit: uint) {
    for i in range(1, limit) {
        (write!(file,
"impl<{fs}, {ts}> Fn(Request, Response) -> T{n} for ({fs})
where {bounds} {{
    extern \"rust-call\" fn call(&self, conn: (Request, Response)) -> T{n} {{
        let a = self.0.call(conn);
        {applications}
        return a;
    }}
}}\n\n",
        fs = types("F", i + 2).connect(","),
        ts = types("T", i + 1).connect(","),
        bounds = bounds(i + 1).connect(",\n      "),
        n = i + 1,
        applications = applications(i, "call((a,))").connect("\n        "),
        )).unwrap()
    }
}

fn types(prefix: &str, limit: uint) -> Vec<String> {
    range(1, limit + 1).map(|n| format!("{}{}", prefix, n)).collect()
}

fn bounds(limit: uint) -> Vec<String> {
    let mut out = vec!["F1: Fn(Request, Response) -> T2".into_string()];
    out.extend(range(2, limit + 1)
        .map(|i| format!("F{}: Fn(T{}) -> T{}", i, i - 1, i)));
    out
}

fn applications(limit: uint, op: &str) -> Vec<String> {
    range(2, limit + 1)
        .map(|n| format!("let a = self.{}.{};", n, op))
        .collect()
}

