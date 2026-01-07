use std::env;
use std::io::{self, Write};

// Note: This version is not actually POSIX-compliant, and the filename is a misnomer. See README.md for more information.

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let joined;

    args.remove(0);

    let output_str = if args.len() == 0 {
        b"y\n"
    } else {joined = args.join(" ") + "\n"; joined.as_bytes()};

    let mut io_handle = io::stdout().lock();
    let buf = output_str.repeat(16384/output_str.len());
    loop {if io_handle.write_all(&buf).is_err() {return;}}
}
