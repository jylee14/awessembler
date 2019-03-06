use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("expected at least 1 assembly file with .s extension!");
        return ;
    }


    let args = args.into_iter()
        .filter(|x| x.contains(".s"))
        .collect::<Vec<String>>();

    if args.len() < 1 {
        eprintln!("expected at least 1 assembly file with .s extension!");
        return ;
    }

    match awessembler::process_command_args(&args){
        Err(e) => {
            eprintln!("Assembly failed!\nAt line {}, {}",
                      e.line_number, e.message);
        },
        _ => println!("DONE"),
    };
}
