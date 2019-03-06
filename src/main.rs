use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("expected at least 1 assembly file!");
        return ();
    }

    match awessembler::process_args(&args[1..]){
        Err(e) => {
            eprintln!("Assembly failed!\nAt line {}, {}",
                      e.line_number,
                      e.message);
        },
        _ => println!("DONE"),
    };
}
