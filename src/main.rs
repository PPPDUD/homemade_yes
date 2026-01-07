use std::env;
use std::io::{self, Write};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let joined;

    args.remove(0);

    let pos_help = args.iter().position(|n| n == "--help").unwrap_or(usize::MAX);
    let pos_version = args.iter().position(|n| n == "--version").unwrap_or(usize::MAX);

    if pos_help < pos_version {
        println!("Outputs the string provided in the arguments forever. If no string is given, outputs 'y' repeatedly.");
        println!("For version information, invoke with --version.");
        return;
    }

    if pos_help > pos_version {
        println!("yes v0.4 (Mojavesoft implementation); should be compatible with GNU yes v9.5 and uutils yes 0.2.2");
        println!("Copyright January 2026.");
        return;
    }

    let output_str = if args.len() == 0 {
        b"y\n"
    } else {joined = args.join(" ") + "\n"; joined.as_bytes()};

    let mut io_handle = io::stdout().lock();
    let buf = output_str.repeat(16384/output_str.len());
    loop {if io_handle.write_all(&buf).is_err() {return;}}
}
