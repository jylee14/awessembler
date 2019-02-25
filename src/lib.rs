#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fs;
use std::str;
use std::error::Error;
use std::fmt;

const DEBUG:bool = false;

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
        m.insert("BLE",     "1_1001");
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
        if line.starts_with("//") || line.contains(":") || line.is_empty() {
            machine_codes.push(String::from(line));
            continue;
        }

        let processed_line = match process_line(line){
            Ok(l) => l,
            Err(msg) => {
                return Err(AsmErr::new((index + 1)as u32, msg));
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
    if DEBUG{
        println!("processing: {}", line);
    }
    let mut processed_line = String::new();

    let words:Vec<&str> = line.split(" ").collect();    //get all the components of the line
    let instruction = words[0].to_ascii_uppercase();
    let &machine_code = INSTRUCTION.get(instruction.as_str()).expect("invalid assembly instruction!");

    let iterim_machine_code = String::from(machine_code);
    let iterim_machine_code = iterim_machine_code.replace("_", "");
    let machine_code = iterim_machine_code.as_str();

    processed_line.push_str(machine_code);
    //handle single argument instructions first
    if instruction == "CLR" || instruction  == "BA" || instruction == "BR" || instruction == "HALT"{
        return Ok(String::from(machine_code));
    }

    let arg = words[1].as_bytes();
    let mut number = String::from_utf8_lossy(&arg[1..]);
    let radix = if number.contains("0x") || number.contains("0X"){
        number = String::from_utf8_lossy(&arg[3..]);
        16
    }else if number.contains("0b") || number.contains("0B"){
        number = String::from_utf8_lossy(&arg[3..]);
        2
    }else{
        10
    };

    if DEBUG{
        eprintln!("number detected: {}, radix {}", number, radix);
    }

    match arg[0]{
        b'r' | b'R' | b'#' => {
            let int_value = match i16::from_str_radix(&number, radix){
                Ok(v) => v,
                Err(_) => return Err("invalid number!")
            };
            let int_value = int_value as i8;
            let start_index = machine_code.len() - 1;

            let mut binary_rep = format!("{:b}", int_value);
            while binary_rep.len() < 8{
                binary_rep = "0".to_owned() + &binary_rep;
            }
            processed_line.push_str(&binary_rep[start_index..]);
        },
        b'$' => {
            let special_reg = String::from_utf8_lossy(&arg[1..]).to_ascii_uppercase();
            if special_reg == "ZERO" {
                processed_line.push_str("1111");
            }else {
                return Err("Unknown special register specified");
            }
        },
        b'['=> {
            match words[1].find("]"){
                None => return Err("Unmatched bracket!"),
                _ => ()
            };

            match arg[1] {
                b'r' | b'R' => {
                    let max = if arg.len()-1 > 2 {
                        arg.len() - 1
                    }else{
                        2
                    };
                    let number = String::from_utf8_lossy(&arg[2..max]);
                    let int_value = match number.parse::<u8>(){
                        Ok(i) => i,
                        Err(_) => return Err("parse failed!")
                    };


                    let start_index = machine_code.len() - 1;
                    let mut binary_rep = format!("{:b}", int_value);
                    while binary_rep.len() < 8{
                        binary_rep = "0".to_owned() + &binary_rep;
                    }
                    processed_line.push_str(&binary_rep[start_index..]);
                },
                _ => return Err("Invalid use of ldr/str syntax"),
            }

        },
        _ => return Err("Encountered unknown symbol"),
    }

    if line.contains("//") {    //theres a comment
        let comment = line.find("//").unwrap(); //guaranteed to contain '//'
        let comment = &line[comment..];
        processed_line.push_str(format!("\t\t{}", comment).as_str());
    }

    //assert!(processed_line.len() == 9);
    Ok(processed_line)
}