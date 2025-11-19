use std::env;
use std::io::{self, Write};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let joined;

    args.remove(0);

    let output_str = if args.len() == 0 {
        b"y\n"
    } else {joined = args.join(" ") + "\n"; joined.as_bytes()};

    let mut io_handle = io::stdout().lock();
    let buf = output_str.repeat(4096);
    loop {if io_handle.write_all(&buf).is_err() {return;}}
}
