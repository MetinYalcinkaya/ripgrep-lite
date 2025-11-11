use std::env;

use ripgrep_lite::search;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_args = args.len();

    if num_args != 3 {
        // print to stderr and exit
        eprintln!("Invalid number of args");
        eprintln!("Usage: rglt <pattern> <file>");
        std::process::exit(2);
    }

    let pattern = &args[1];
    let path = &args[2];

    // read file
    let file = std::fs::read_to_string(path).unwrap();

    let lines = search(pattern, &file);

    for (line_no, line) in lines {
        println!("{}: {}", line_no, line);
    }
}
