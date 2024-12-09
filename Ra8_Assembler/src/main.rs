use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::string::String;

fn lexer(line: String) -> HashMap<&'static str, String> {
    let mut token = HashMap::new(); //the hashmap in which all the tokens will be stored
    let mut string = line.clone(); //the string that will under go tokenizations
    let strrrr = line.clone(); //the string that will under go tokenizations

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
    // println!("{:?} => {:?}", strrrr, token);
    return token;
}

fn Codegen(tokens: Vec<HashMap<&str, String>>, instruction_table: Vec<Opcodes>) {
    for token in tokens {
        if let Some(instr) = instruction_table
            .iter()
            .find(|instr| instr.instruction == *token.get("Instruction").unwrap_or(&"".to_string()))
        // Dereferencing the Option
        {
            println!("Found instruction: {:?}", instr);
            // Do something with `instr`, e.g., print the machine code
            println!("Machine Code: {}", instr.machine_code);
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
    bytes: u8,
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
    Codegen(tokens, instruction_table);

    Ok(())
}
