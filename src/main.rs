mod instruction;
mod parser;
mod token;
mod lexer;

use lexer::*;
use parser::*;
use std::fs::*;
use regex::Regex;
use std::env::args;
use std::io::Write;

fn remove_comments(value: String) -> String{
    let regex: Regex = Regex::new(r"(/\*([^*]|[\r\n]|(\*+([^*/]|[\r\n])))*\*+/)|(//.*)").unwrap();
    let value_no_c_comment: String = regex.replace_all(value.as_str(), "").to_string();
    let regex_asm: Regex = Regex::new(r";.*").unwrap();
    regex_asm.replace_all(value_no_c_comment.as_str(), "").to_string()
}

fn main(){
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_input> (file_output)", args[0]);
        return;
    }
    let source: String = remove_comments(std::fs::read_to_string(args[1].clone()).unwrap());
    if cfg!(debug_assertions){
        println!("{}", source.clone());
    }
    let mut lexer: Lexer = Lexer::new(source);
    match lexer.scan(){
        Ok(tokens) => {
            let mut parser: Parser = Parser::new(tokens, lexer.get_label_map());
            match parser.parse(){
                Ok(instructions) => {
                    if cfg!(debug_assertions){
                        println!("Result: ");
                    }
                    let mut bytes: Vec<u8> = Vec::new();
                    for instruction in instructions{
                        let value: u16 = instruction.as_u16();
                        bytes.push((value >> 8) as u8);
                        bytes.push((value & 0xFF) as u8);
                        if cfg!(debug_assertions){
                            println!(
                                "\tInstruction: {},\tHex: {:#06X},\tBin: {:#018b}",
                                instruction.get_as_string(),
                                value.clone(),
                                value.clone()
                            );
                        }
                    }
                    let mut file_path: String = args[1].clone().replace(".asm", ".bin").replace(".s", ".bin");
                    if let Some(file_name) = args.get(2){
                        file_path = (*file_name).clone();
                    }
                    let mut file: File = File::create(file_path).unwrap();
                    file.write_all(&bytes).unwrap();
                },
                Err(e) => {
                    e.report();
                }
            }
        },
        Err(e) => {
            e.report();
        }
    }
}
