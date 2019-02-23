#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args:Vec<String> = env::args().collect("");

    if args.len() < 2 {
        eprintln!("expected at least 1 assembly file!");
        return ();
    }

    process_args(&args[1..]).unwrap();
}

lazy_static!{
    static INSTRUCTION: HashMap<&'static str, u16> {
        let mut m = HashMap::new();
        m.insert("MOV",     0b0);
        m.insert("AND",     0b1_0000_0);
        m.insert("ORR",     0b1_0000_1);
        m.insert("LSL",     0b1_00001_0);
        m.insert("LSR",     0b1_00001_1);
        m.insert("ADD",     0b1_0010_0);
        m.insert("ADC",     0b1_0010_1);
        m.insert("SUB",     0b1_00101_0);
        m.insert("SBC",     0b1_00101_1);
        m.insert("LDR",     0b1_0100_0);
        m.insert("STR",     0b1_0100_1);
        m.insert("WRT",     0b1_0101);
        m.insert("RDR",     0b1_0110);
        m.insert("MVN",     0b1_0111);
        m.insert("CMP",     0b1_1000);
        m.insert("BLT",     0b1_1001);
        m.insert("BGT",     0b1_1010);
        m.insert("BEQ",     0b1_1011);
        m.insert("BNE",     0b1_1110);
        m.insert("CLZ",     0b1_11101_0);
        m.insert("BA",      0b1_11101_1000);
        m.insert("BR",      0b1_11101_1001);
        m.insert("HALT",    0b1_11101_1111);
    }
}


fn process_args(args: &[String]) -> Result<(), &str>{
    for &arg in args.iter(){
        let file_content = fs::read_to_string(arg).unwrap();
        process_file(file_content);
    }

    return Ok(())
}

fn process_file(filestring: String){
    let machine_code = vec![];
    for line in filestring.lines(){
        
    }
}