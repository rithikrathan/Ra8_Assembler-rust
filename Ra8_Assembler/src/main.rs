use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // let mut _3byteInstructions = vec![
    //     "0x0032", "0x0034", "0x0035", "0x0036", "0x0037", "0x0038", "0x0045", "0x0048", "0x0049",
    //     "0x004A", "0x004B", "0x004C", "0x004D", "0x004E", "0x004F", "0x0050", "0x0051", "0x0052",
    //     "0x0053", "0x0054", "0x0055", "0x0056", "0x0057", "0x0058", "0x0059",
    // ];
    // println!("{:?}", _3byteInstructions);
    let filename =
        "/home/rathanthegreatlol/Desktop/projects/Ra8_Assembler/Example_Assembly_code/FACTORIAL.asm"; //PUT THE FILE PATH OF THE ASSEMBLY CODE
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line_content) => lexer(line_content),
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(())
}

fn lexer(line: String) {
    //ignore empty lines
    if line.trim().is_empty() {
        return;
    }

    let mut token = HashMap::new(); //the hashmap in which all the tokens will be stored
    let mut string = line.clone(); //the string that will under go tokenizations
    let strrrr = line.clone(); //input string of the assembly program line
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
        } else{
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
    println!("{:?} => {:?}", strrrr, token);
}
