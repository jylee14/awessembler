#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fs;
use std::str;
use std::error::Error;
use std::fmt;

const DEBUG:bool = false;

//asm -> (machine code, type) map
lazy_static!{
    static ref INSTRUCTION: HashMap<&'static str, (&'static str, &'static str)> = {
        let mut m = HashMap::new();
        m.insert("MOV" , ("0",         "C"));
        m.insert("BLE" , ("11001",     "C"));
        m.insert("BGT" , ("11010",     "C"));
        m.insert("BEQ" , ("11011",     "C"));
        m.insert("BNE" , ("11110",     "C"));
        m.insert("LSL" , ("100010",    "C"));
        m.insert("LSR" , ("100011",    "C"));

        m.insert("LDR" , ("101000",    "M"));
        m.insert("STR" , ("101001",    "M"));

        m.insert("AND" , ("100000",    "R3"));
        m.insert("ORR" , ("100001",    "R3"));
        m.insert("ADD" , ("100100",    "R3"));
        m.insert("ADC" , ("100101",    "R3"));
        m.insert("SUB" , ("100110",    "R3"));
        m.insert("SBC" , ("100111",    "R3"));
        m.insert("CLZ" , ("1111010",   "R3"));

        m.insert("CMP" , ("11000",     "R4"));
        m.insert("WRT" , ("10101",     "R4"));
        m.insert("RDR" , ("10110",     "R4"));
        m.insert("MVN" , ("10111",     "R4"));

        m.insert("CLR" , ("000000000", "O"));
        m.insert("BA"  , ("111111000", "O"));
        m.insert("BR"  , ("111111001", "O"));
        m.insert("HALT", ("111111111", "O"));

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

impl Error for AsmErr{ }
impl fmt::Display for AsmErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Assembly failed!\nline: {}\t{}", self.line_number, self.message)
    }
}

pub fn process_command_args(args: &[String]) -> Result<(), AsmErr>{
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
            continue;
        }

        if DEBUG{
            println!("processing line {}: {}", index+1, line);
        }

        let processed_line = match process_line(line){
            Ok(l) => l,
            Err(msg) => {
                return Err(AsmErr::new((index + 1)as u32,
                                       format!("{}\n{}", line, msg).as_str()));
            }
        };
        machine_codes.push(processed_line);
    }

    let processed_code = machine_codes.join("\n");
    let filename = filename.replace(".s", ".m");
    match fs::write(filename, processed_code){
        Err(_) => return Err(AsmErr::new(0, "failed to write file to disk!")),
        _ => (),
    };

    Ok(())
}

fn process_line(line: &str)->Result<String, &'static str>{
    let mut processed_line = String::new();

    let words:Vec<&str> = line.split(" ").collect();    //get all the components of the line
    let instruction = words[0].to_ascii_uppercase();
    let &machine_code = match INSTRUCTION.get(instruction.as_str()){
        Some((v, _)) => v,
        None => return Err("invalid assembly instruction!"),
    };

    processed_line.push_str(machine_code);

    //handle single argument instructions first
    if instruction == "CLR" || instruction  == "BA" || instruction == "BR" || instruction == "HALT"{
        return Ok(processed_line);
    }

    let arg = match process_arg(&instruction, words[1].trim().as_bytes()){
        Ok(a) => a,
        Err(e) => return Err(e),
    };
    let arg = if arg.len() + machine_code.len() > 9{ // need to trim
        let start = arg.len() + machine_code.len() - 9;
        &arg.as_bytes()[start..]
    }else{
        arg.as_bytes()
    };

    let final_arg = String::from_utf8_lossy(arg);
    processed_line.push_str(&final_arg);

    assert_eq!(processed_line.len(), 9);
    Ok(processed_line)
}

fn process_arg(asm: &str, arg: &[u8])->Result<String, &'static str>{
    let arg_type = INSTRUCTION.get(asm).unwrap().1;  //should NEVER fail here
    let mut number;
    match arg_type{
        "C" => {
            if arg[0] != b'#' {
                return Err("usage: ASM #const");
            }

            number = String::from_utf8_lossy(&arg[1..]);
            let radix = if number.contains("0x") || number.contains("0X"){
                number = String::from_utf8_lossy(&arg[3..]);
                16
            }else if number.contains("0b") || number.contains("0B"){
                number = String::from_utf8_lossy(&arg[3..]);
                2
            }else{
                10
            };

            let int_value= match i16::from_str_radix(&number, radix) {
                Ok(v) => v,
                Err(_) => return Err("invalid number!")
            } as i8;

            let binary_rep = format!("{number:>0width$b}", number = int_value, width=8);
            return Ok(binary_rep);
        },

        "M" => {
            if arg[0] != b'['{
                return Err("usage: LDR/STR [r0-7]");
            }

            if !(arg.iter().any(|x| *x == ']' as u8)){
                return Err("Unmatched bracket!");
            };

            match arg[1]{
                b'r' | b'R' => {
                    number = String::from_utf8_lossy(&arg[2..arg.len()-1]);
                    let int_value = match number.parse::<u8>(){
                        Ok(i) => i,
                        Err(_) => return Err("parse failed!")
                    };

                    let binary_rep = format!("{number:>0width$b}", number = int_value, width=3);
                    Ok(binary_rep)
                },
                _ => return Err("Invalid use of ldr/str syntax"),
            }
        },

        "R3" | "R4" => {
            number = String::from_utf8_lossy(&arg[1..]);
            match arg[0] {
                b'r' | b'R' => {
                    let int_value = match number.parse::<u8>(){
                        Ok(v) => v,
                        Err(_) => return Err("invalid number!")
                    };

                    let width = if arg_type == "R3" { 3 }else{ 4 };
                    let binary_rep = format!("{num:>0width$b}", num = int_value, width = width);
                    Ok(binary_rep)
                },
                b'$' => {
                    let special_reg = String::from_utf8_lossy(&arg[1..]).to_ascii_uppercase();
                    if special_reg == "ZERO" {
                        Ok(String::from("1111"))
                    } else {
                        return Err("Unknown special register specified");
                    }
                },
                _ => {
                    return Err("encountered unexpected symbol!");
                }
            }
        },

        _ => panic!("NANI?????"),    //this shouldnt happen
    }
}