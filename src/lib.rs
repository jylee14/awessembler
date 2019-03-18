#[macro_use]
extern crate lazy_static;

use std::fs;
use std::fmt;
use std::str;
use std::error::Error;
use std::collections::HashMap;

use linked_hash_map::LinkedHashMap;

#[derive(Debug)]
pub struct AsmErr {
    pub line_number: u32,
    pub message: String,
}

impl AsmErr {
    fn new(line_number: u32, message: &str) -> AsmErr {
        AsmErr {
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

/// process_command_args
/// public facing function. called by the main to parse commandline
/// it will take each commandline argument and process the file, writing the
/// resulting machine code to the disk upon successful conversion.
///
/// # Arguments
/// * `args` - list of assembly file names
///
/// # Return types
/// * `Result<(), AsmErr>`
///
/// # Error condition
/// if passed in filename does NOT exist in the disk
pub fn process_command_args(args: &[String]) -> Result<(), AsmErr>{
    for arg in args.iter(){
        let file_content = match fs::read_to_string(&arg){
            Ok(str) => str,
            Err(_) => return Err(AsmErr::new(0,
                                             "failed to read assembly file!")),
        };

        let file_content = file_content.to_ascii_uppercase();
        process_file(arg, file_content)?;
    }

    return Ok(())
}

/// process_file
/// goes through each line in the assembly file and converts it to machine code file
/// & writes the result to disk
///
/// # Arguments
/// * `filename` - filename
/// * `filestring` - a long string that is the content of the assembly file
///
/// # Return Type
/// Result<(), AsmErr>
///
/// # Error condition
/// assembly file processing fails
/// writing to disk fails
fn process_file(filename: &String, filestring: String)->Result<(), AsmErr>{
    //preprocess the file so that all the code & labels are in place
    let preprocessed_assembly = preprocess_assembly(&filestring);
    let (lines_asm, labels_lines) = transform_processed_assembly(preprocessed_assembly);

    let mut machine_codes:Vec<String> = vec![];
    for (line_num, line) in lines_asm.iter(){
        if DEBUG{
            //eprintln!("PROCESSING:\t{}", line);
        }

        if line.contains(":"){
            continue;
        }

        //does this mov have a label or a constant?
        if line.contains("MOV") || (!line.contains("BR") && line.starts_with("B")){
            let asm_words:Vec<&str> = line.split(" ").collect();
            let asm_inst = asm_words[0];    //this is the instruction
            let asm_label = asm_words[1];   //this is the label

            let processed_line;
            if line.contains("#"){
                if DEBUG{
                    eprintln!("LINE:\t\t{}", line);
                }

                processed_line = match process_line(line) {
                    Ok(l) => l,
                    Err(e) => return Err(AsmErr::new(*line_num, e)),
                };
            }else {
                if DEBUG{
                    eprintln!("LABELED LINE:\t{}", line);
                }

                let asm_label = format!("{}:", asm_label);
                let label_line_num = match labels_lines.get(&asm_label) {
                    Some(&l) => l as u32,
                    None => return Err(AsmErr::new(line_num + 1, "Invalid label!")),
                };

                let mut line_diff = label_line_num as i32 - *line_num as i32;
                if line.starts_with("B"){
                    line_diff += 1;
                }

                let new_inst = format!("{} #{}", asm_inst, line_diff);   //mov with appropriate distance set

                if DEBUG {
                    eprintln!("new inst:\t{}", new_inst);
                }

                processed_line = match process_line(&new_inst) {
                    Ok(l) => l,
                    Err(e) => return Err(AsmErr::new(*line_num, e)),
                };
            }

            machine_codes.push(processed_line);
        }else{
            let processed_line = match process_line(&line) {
                Ok(l) => l,
                Err(e) => return Err(AsmErr::new(*line_num, e)),
            };

            machine_codes.push(processed_line);
        }
    }


    let processed_code = machine_codes.join("\n");
    let filename = filename.replace(".s", ".m");
    match fs::write(filename, processed_code){
        Err(_) => return Err(AsmErr::new(0, "failed to write file to disk!")),
        _ => (),
    };

    Ok(())
}

/// preprocess_assembly
/// go through the lines of the assembly file, getting the labels and
/// adding the assembly code/label into the map
///
/// # Argument
/// * `filestring` - string that contains all the assembly.
///
/// # Return type
/// * `HashMap<u32, String>` - hashmap of line number/assembly
///
/// # Error condition
/// NONE
fn preprocess_assembly(filestring: &String)->Vec<String>{
    let mut assembly = vec![];
    for line in filestring.lines(){
        if line.starts_with("//") {
            continue
        }

        let line = line.to_ascii_uppercase();
        let words = line.as_bytes();

        let mut comment_idx= -1;
        'inner: for (idx, &word) in words.iter().enumerate(){
            if word == b'/' {
                comment_idx = idx as i8;
                break 'inner;
            }
        };

        let line = String::from(if comment_idx == -1{
            &line
        }else{
            &line[..(comment_idx as usize)]
        });

        let line = String::from(line.trim());
        if line.contains(":") {
            if DEBUG{
                eprintln!("LABEL FOUND:\t{}", line);
            }
            assembly.push(line);
        }else {
            if line.starts_with("BR") {
                let branch_and_label: Vec<&str> = line.split(" ").collect();
                if branch_and_label.len() > 1 {
                    let labeled_move = format!("MOV {}", branch_and_label[1]);
                    if DEBUG {
                        eprintln!("labeled MOV:\t{}", labeled_move);
                    }
                    assembly.push(labeled_move);
                }
                assembly.push(String::from("BR"));
            }else if line.starts_with("B") {
                let branch_and_label: Vec<&str> = line.split(" ").collect();
                if branch_and_label[1].starts_with("#") {
                    assembly.push(line);
                }else{
                    let labeled_jump = format!("{} {}", branch_and_label[0], branch_and_label[1]);
                    if DEBUG {
                        eprintln!("labeled jump:\t{}", labeled_jump);
                    }
                    assembly.push(labeled_jump);
                }
            }else {
                assembly.push(String::from(line));
            }
        }
    }

    assembly
}

/// transform_processed_assembly
/// assign line numbers to each assembly or label
///
/// # Argument
/// * `assembly` - vector of strings where each element is a line of assembly code
///
/// # Return type
/// * `LinkedHashMap<u32, String>` - line number ordered hashmap of assembly instructions
/// * `HashMap<String, u32>` - label - line number hash map
///
/// # Error condition
/// NONE
fn transform_processed_assembly(assembly: Vec<String>) -> (LinkedHashMap<u32, String>, HashMap<String, u32>){
    let mut labels = HashMap::new();
    let mut map = LinkedHashMap::new();

    let mut line_num = 0;
    for line in assembly.into_iter(){
        if line.is_empty(){
            continue;
        }

        //if label, dont give it a line number and dont add it to the map
        if line.contains(":"){
            labels.insert(line, line_num-1);
        }else{
            map.insert(line_num, line);
            line_num += 1;
        }
    }

    (map, labels)
}

/// process_line
/// process a SINGLE line of assembly instruction
/// goes through the assembly map, figures out its machine code
/// & passes the argument to process_args function
///
/// # Argument
/// * `line` - assembly line to process
///
/// # Return Type
/// * `Result<String, &'static str>`
/// Result holding properly processed line (9 bit machine code) or
/// static string stating failure reason
///
/// # Error condition
/// invalid assembly instruction passed in
/// invalid arguments passed in
fn process_line(line: &str)->Result<String, &'static str>{
    let words:Vec<&str> = line.split(" ").collect();    //get all the components of the line
    let instruction = words[0].to_ascii_uppercase();
    let &machine_code = match INSTRUCTION.get(instruction.as_str()){
        Some((v, _)) => v,
        None => return Err("invalid assembly instruction!"),
    };

    let mut processed_line = String::new();
    processed_line.push_str(machine_code);

    //easy single argument instructions
    if instruction == "CLR" || instruction == "HALT" || instruction == "BA" || instruction == "BR"{
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

/// process_arg
/// process args of the given assembly code and truncates to appropriate length
///
/// # Argument
/// * `asm` - assembly instruction (line)
/// * `arg` - arguments to asm. in byte array
///
/// # Return Type
/// * `Result<String, &'static str>`
/// Result holding properly processed args (in bitstring) or
/// static string stating failure reason
///
/// # Error condition
/// invalid use of assembly instruction
/// number too large
/// syntactic errors
fn process_arg(asm: &str, arg: &[u8])->Result<String, &'static str>{
    let arg_type = INSTRUCTION.get(asm).unwrap().1;  //should NEVER fail here
    let mut number;
    match arg_type{
        "C" => {
            if arg[0] != b'#' {
                return Err("usage: ASM #const");
            }

            number = String::from_utf8_lossy(&arg[1..]);
            let radix = if number.contains("0X"){
                number = String::from_utf8_lossy(&arg[3..]);
                16
            }else if number.contains("0B"){
                number = String::from_utf8_lossy(&arg[3..]);
                2
            }else{
                10
            };

            let int_value= match i16::from_str_radix(&number, radix) {
                Ok(v) => v,
                Err(_) => return Err("invalid number!")
            } as i8;

            if asm.contains("B") && (int_value > 7 || int_value < -8){
                return Err("Constant too large for instruction!");
            }

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
                b'R' => {
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
                b'R' => {
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
