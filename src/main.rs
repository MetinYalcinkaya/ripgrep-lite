use std::env;

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
}
