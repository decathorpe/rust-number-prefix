extern crate number_prefix;
use number_prefix::{decimal_prefix, binary_prefix, Standalone, Prefixed};

fn main() {

    // part one
    let mut n = 1_f64;

    for _ in 0 .. 8 {
        n *= 1000_f64;

        let decimal = match decimal_prefix(n) {
            Prefixed(prefix, n) => format!("{:.3} {}B", n, prefix),
            Standalone(_)       => unreachable!(),
        };

        let binary = match binary_prefix(n) {
            Prefixed(prefix, n) => format!("{:.3} {}B", n, prefix),
            Standalone(n)       => format!("{} bytes still", n),
        };

        println!("{:26} bytes is {}, but only {:10}", n, decimal, binary);
    }

    println!();


    // part two
    let mut n = 1_f64;

    for _ in 0 .. 8 {
        n *= 1024_f64;

        let decimal = match decimal_prefix(n) {
            Prefixed(prefix, n) => format!("{:.3} {}B", n, prefix),
            Standalone(_)       => unreachable!(),
        };

        let binary = match binary_prefix(n) {
            Prefixed(prefix, n) => format!("{:.3} {}B", n, prefix),
            Standalone(_)       => unreachable!(),
        };

        println!("{:26} bytes is {} and {:10}", n, binary, decimal);
    }
}
