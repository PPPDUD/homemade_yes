use std::env;
use std::io::{self, Write};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let joined;

    args.remove(0);

    let pos_help = args.iter().position(|n| n == "--help");
    let pos_version = args.iter().position(|n| n == "--version");

    // Begin compatibility with GNU extensions
    if pos_help < pos_version {
        help();
        return;
    }

    if pos_help > pos_version {
        version();
        return;
    }

    let output_str = if args.len() == 0 {
        b"y\n"
    } else {joined = args.join(" ") + "\n"; joined.as_bytes()};

    // End compatibility with GNU extensions
    let mut io_handle = io::stdout().lock();
    let buf = output_str.repeat(4096);
    loop {if io_handle.write_all(&buf).is_err() {return;}}
}

fn help() {
    println!("Outputs the string provided in the arguments forever. If no string is given, outputs 'y' repeatedly.");
    println!("For version information, invoke with --version.");
}

fn version() {
    println!("yes (Mojavesoft implementation); should be compatible with GNU yes v9.5");
    println!("Copyright November 2025.");
}
