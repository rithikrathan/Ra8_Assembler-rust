use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::string::String;
use std::u16;

fn lexer(line: String) -> HashMap<&'static str, String> {
    let mut token = HashMap::new(); //the hashmap in which all the tokens will be stored
    let mut string = line.clone(); //the string that will undero tokenizations
    let strrrr = line.clone(); //the string that will not undergo tokenizations just to compare
                               //before and after tokenisation

    //ignore empty lines
    if line.trim().is_empty() {
        return token;
    }
    //STAGE1: handle comments and remove them from the program line
    if let Some(pos) = string.find(";") {
        string.truncate(pos);
    }
    //STAGE2: handle label definitions
    if let (Some(spos), Some(cpos)) = (string.find("$"), string.find(":")) {
        if spos < cpos {
            let label = string[spos + 1..cpos].to_string();
            string = string[cpos + 2..].to_string();
            // token.insert("Instruction", string);
            token.insert("Label", label);
        } else {
            eprintln!("Error: invalid use ':' in {:?}", strrrr);
        }
    }
    //STAGE3: handle instructions,immediate values and references
    if let Some(zpos) = string.find("0x") {
        let hex = string[zpos..].trim().to_string();
        string.truncate(zpos - 1);
        token.insert("Instruction", string.clone());
        token.insert("Hex", hex);
    } else if let Some(apos) = string.clone().find("$") {
        let reff = string[1 + apos..].trim().to_string();
        string.truncate(apos - 1);
        token.insert("Instruction", string);
        token.insert("Ref", reff);
    } else {
        token.insert("Instruction", string.trim().to_string());
    }
    println!("{:?} => {:?}", strrrr, token); //for debugging
    return token;
}

//NOTE: idk something is wrong with calculating address for labels (i think)
//I didn't test the output machine code with the emulator but it appears to calculate addresses
//correctly
fn set_label_table(
    tokens: &Vec<HashMap<&str, String>>,
    instruction_table: &Vec<Opcodes>,
) -> HashMap<String, u16> {
    let mut label_tabel = HashMap::new();
    let mut addr: u16 = 0; //initial address(if i add ORI assembler directive then its argument is
                           //assigned to this variable)
    for token in tokens {
        //if there is a label definition in the token then add it to the hashmap with addr variable
        //as its value
        if let Some(label) = token.get("Label") {
            label_tabel.insert(label.to_string(), addr);
        }
        //for every new instruction check the instruction table for its byte size and add it to the
        //addr variable
        if let Some(instr) = instruction_table
            .iter()
            .find(|instr| instr.instruction == *token.get("Instruction").unwrap_or(&"".to_string()))
        // Dereferencing the Option
        {
            addr += instr.bytes
        }
    }
    println!("LABEL TABLE:"); //for debugging
    println!("{:?}", label_tabel); //for debugging
    return label_tabel;
}

fn codegen(
    tokens: &Vec<HashMap<&str, String>>,
    instruction_table: &Vec<Opcodes>,
    label_tabel: &HashMap<String, u16>,
) {
    let _3byte_instructions = vec![
        "0x0032", "0x0034", "0x0035", "0x0036", "0x0037", "0x0038", "0x0045", "0x0048", "0x0049",
        "0x004A", "0x004B", "0x004C", "0x004D", "0x004E", "0x004F", "0x0050", "0x0051", "0x0052",
        "0x0053", "0x0054", "0x0055", "0x0056", "0x0057", "0x0058", "0x0059",
    ];
    println!("MACHINE CODE:"); //for debugging
    for token in tokens {
        if let Some(instr) = instruction_table
            .iter()
            .find(|instr| instr.instruction == *token.get("Instruction").unwrap_or(&"".to_string()))
        {
            //for every new instruction first print its machine code
            println!("{}", instr.machine_code);
            //after printing the machine code check if the immediate value exitsts and if it does
            //then and it is an 8bit value then print it as it is, if it is a 16bit value then
            //split it into high and low bytes and then print them
            if let Some(hex) = token.get("Hex") {
                if _3byte_instructions.contains(&instr.machine_code.as_str()) {
                    let hex_val: u16 =
                        u16::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap();
                    let highbyte = hex_val >> 8;
                    let lowbyte = hex_val & 255;
                    println!("{}", format!("0x{:04X}", lowbyte));
                    println!("{}", format!("0x{:04X}", highbyte));
                } else {
                    println!("{}", hex);
                }
            }
        }
        //check if there is any label references in the tokens if there is then split the address
        //into high and low bytes and print them
        if let Some(reference) = token.get("Ref") {
            let addr: u16 = *label_tabel.get(reference).unwrap();
            // println!("{:?}", addr);
            let highbyte = addr >> 8;
            let lowbyte = addr & 255;
            println!("{}", format!("0x{:04X}", lowbyte));
            println!("{}", format!("0x{:04X}", highbyte));
        }
    }
}

#[derive(Debug, Deserialize)]
struct Opcodes {
    #[serde(rename = "INSTRUCTION")]
    instruction: String,
    #[serde(rename = "MACHINE CODE")]
    machine_code: String,
    #[serde(rename = "BYTES")]
    bytes: u16,
}

fn main() -> io::Result<()> {
    //~~~~~~~~~~~~LOADS THE ASSEMBLY FILE~~~~~~~~~~~~~//

    let mut tokens: Vec<HashMap<&'static str, String>> = Vec::new();
    let filename ="/home/rathanthegreatlol/Desktop/projects/Ra8_Assembler/Example_Assembly_code/FACTORIAL.asm"; //PUT THE FILE PATH OF THE ASSEMBLY CODE
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                let token = lexer(line_content);
                tokens.push(token);
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    //~~~~~~~~~~~~LOADS INSTRUCTIONS TABLE~~~~~~~~~~~~~//

    const JSON_DATA: &[u8] = include_bytes!("../Instructions.json");
    let instruction_table: Vec<Opcodes> =
        serde_json::from_slice(JSON_DATA).expect("Failed to parse JSON");

    //~~~~~~~~~~~~DOES SOMETHING WITH IT~~~~~~~~~~~~~~~//
    let labelTabel = set_label_table(&tokens, &instruction_table);
    codegen(&tokens, &instruction_table, &labelTabel);
    Ok(())
}
