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

    // lexer(String::from("$LOOP: MOV B A ;this is a comment"));
    // lexer(String::from("MOV B A ;this is a comment"));
    // lexer(String::from("LDI 0x4000 ;this is a comment"));
    // lexer(String::from("CMP ;this is a comment"));

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

// fn lexer(line: String) -> HashMap<&str, String> {
fn lexer(line: String) {
    //ignore empty lines
    if line.trim().is_empty() {
        return;
    }

    let mut token = HashMap::new();
    let mut string = line;
    //handle comments and remove them from the program line
    if let Some(pos) = string.find(";") {
        string.truncate(pos);
    }
    //handle label definitions
    if let (Some(spos), Some(cpos)) = (string.find("$"), string.find(":")) {
        if spos < cpos {
            let label = string[spos + 1..cpos].to_string();
            string = string[cpos + 1..].to_string();
            // token.insert("Instruction", string);
            token.insert("Label", label);
        }
    }
    //handle instructions
    let parts: Vec<&str> = string.split_whitespace().collect();

    let mut opcode = "";
    let mut arg1: Option<String> = None;
    let mut arg2: Option<String> = None;

    match parts.as_slice() {
        [op] => {
            opcode = op;
        }
        [op, a1] => {
            opcode = op;
            arg1 = Some(a1.to_string());
        }
        [op, a1, a2] => {
            opcode = op;
            arg1 = Some(a1.to_string());
            arg2 = Some(a2.to_string());
        }
        _ => {
            eprintln!("Invalid instruction format.")
        }
    }
    token.insert("opcode", opcode.to_string());
    token.insert("arg1", arg1.unwrap_or_default());
    token.insert("arg2", arg2.unwrap_or_default());
    println!("{:?}", token)
}
