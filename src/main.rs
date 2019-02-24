use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("expected at least 1 assembly file!");
        return ();
    }

    Awessembler::process_args(&args[1..]).unwrap();
}
