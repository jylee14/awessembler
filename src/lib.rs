#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fs;
use std::str;
use std::error::Error;
use std::fmt;

const DEBUG:bool = true;

//asm -> machine code map
lazy_static!{
    static ref INSTRUCTION: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("MOV",     "0_");
        m.insert("CLR",     "0_0000_0000");
        m.insert("AND",     "1_0000_0");
        m.insert("ORR",     "1_0000_1");
        m.insert("LSL",     "1_00001_0");
        m.insert("LSR",     "1_00001_1");
        m.insert("ADD",     "1_0010_0");
        m.insert("ADC",     "1_0010_1");
        m.insert("SUB",     "1_00101_0");
        m.insert("SBC",     "1_00101_1");
        m.insert("LDR",     "1_0100_0");
        m.insert("STR",     "1_0100_1");
        m.insert("WRT",     "1_0101");
        m.insert("RDR",     "1_0110");
        m.insert("MVN",     "1_0111");
        m.insert("CMP",     "1_1000");
        m.insert("BLT",     "1_1001");
        m.insert("BGT",     "1_1010");
        m.insert("BEQ",     "1_1011");
        m.insert("BNE",     "1_1110");
        m.insert("CLZ",     "1_11101_0");
        m.insert("BA",      "1_1111_1000");
        m.insert("BR",      "1_1111_1001");
        m.insert("HALT",    "1_1111_1111");

        m
    };
}

#[derive(Debug)]
pub struct AsmErr{
    pub line_number: u32,
    pub message: String,
}

impl AsmErr{
    fn new(line_number: u32, message: &str) -> AsmErr{
        AsmErr{
            line_number,
            message: String::from(message)
        }
    }
}

impl Error for AsmErr{

}

impl fmt::Display for AsmErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Assembly failed!\nline: {}\t{}", self.line_number, self.message)
    }
}

pub fn process_args(args: &[String]) -> Result<(), AsmErr>{
    for arg in args.iter(){
        let file_content = match fs::read_to_string(&arg){
            Ok(str) => str,
            Err(_) => return Err(AsmErr::new(0,
                                             "failed to read assembly file!")),
        };
        process_file(arg, file_content)?;
    }

    return Ok(())
}

fn process_file(filename: &String, filestring: String)->Result<(), AsmErr>{
    let mut machine_codes = vec![];
    for (index, line) in filestring.lines().enumerate() {
        if line.starts_with("//") { //comment line
            machine_codes.push(String::from(line));
            continue;
        }

        if line.contains(":") || line.is_empty() {
            continue;
        }

        let processed_line = match process_line(line){
            Ok(l) => l,
            Err(msg) => {
                return Err(AsmErr::new(index as u32, msg));
            }
        };
        machine_codes.push(processed_line);
    }

    let processed_code = machine_codes.join("\n");

    let filename = filename.replace(".s", ".m");
    match fs::write(filename, processed_code){
        Err(_) => return Err(AsmErr::new(0,
                               "failed to write file to disk!")),
        _ => (),
    };


    Ok(())
}

fn process_line(line: &str)->Result<String, &'static str>{
    let mut processed_line = String::new();

    let words:Vec<&str> = line.split(" ").collect();    //get all the components of the line
    let instruction = words[0].to_ascii_uppercase();
    let &machine_code = INSTRUCTION.get(instruction.as_str()).expect("invalid assembly instruction!");


    let iterim_machine_code = String::from(machine_code);
    let iterim_machine_code = iterim_machine_code.replace("_", "");
    let machine_code = iterim_machine_code.as_str();

    processed_line.push_str(machine_code);

    //handle single argument instructions first
    if words[0]  == "BA" || words[0] == "BR" || words[0] == "HALT"{
        if DEBUG{
            println!("{}", processed_line);
        }
        return Ok(String::from(machine_code));
    }

    let arg = words[1].as_bytes();
    let mut number = String::from_utf8_lossy(&arg[1..]);
    let radix = if number.contains("0x"){
        number = String::from_utf8_lossy(&arg[3..]);
        16
    }else if number.contains("0b"){
        number = String::from_utf8_lossy(&arg[3..]);
        2
    }else{
        10
    };

    match arg[0]{
        b'r' | b'R' | b'#' => {
            let int_value = u32::from_str_radix(&number, radix).unwrap() as u8;
            let mut binary_rep = format!("{:b}", int_value);
            let start_index = 8 - (9 - machine_code.len());

            while binary_rep.len() < 8{
                binary_rep = "0".to_owned() + &binary_rep;
            }

            processed_line.push_str(&binary_rep[start_index..]);
        },
        b'$' => {
            let special_reg = String::from_utf8_lossy(&arg[1..]).to_ascii_uppercase();
            if special_reg == "ZERO" {
                processed_line.push_str("1110");
            }else if special_reg == "PC" {
                processed_line.push_str("1111");
            }
        },
        _ => return Err("Encountered unknown symbol"),
    }

    if DEBUG{
        println!("{}", processed_line);
    }
    assert!(processed_line.len() == 9);
    Ok(processed_line)
}