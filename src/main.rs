use std::env;

fn main() {
    // for arg in env::args() {
    //     println!("{arg}");
    // }

    let args: Vec<String> = env::args().collect();
    let num_args = args.len();

    if num_args != 3 {
        println!("Invalid number of args");
        println!("Usage: rglt <pattern> <file>");
    }
}
