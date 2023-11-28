mod instruction;
mod parser;
mod token;
mod lexer;
mod args;

use args::*;
use lexer::*;
use std::{fs::*, path::PathBuf};
use clap::Parser;
use std::io::Write;


fn main(){
    let args: AssemblerArgs = AssemblerArgs::parse();
    let source: String = std::fs::read_to_string(args.input_file.clone()).expect("Failed to read file.");
    if cfg!(debug_assertions){
        println!("Source:\n{}", source.clone());
    }
    let mut lexer: Lexer = Lexer::new(source);
    match lexer.scan(){
        Ok(tokens) => {
            let mut parser: parser::Parser = parser::Parser::new(tokens, lexer.get_label_map());
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
                    let file_path: PathBuf = args.get_file_output();
                    let mut file: File = File::create(file_path).expect("Failed to create file!");
                    file.write_all(&bytes).expect("Failed to write into file.");
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
