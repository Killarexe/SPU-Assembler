use crate::token::*;
use crate::instruction::*;
use std::collections::HashMap;

pub struct Parser{
    tokens: Vec<Token>,
    current_token: Token,
    label_map: HashMap<String, u16>,
    instruction_map: HashMap<String, u16>,
    program_size: u16,
    index: usize,
}

pub struct ParserError{
    message: String,
    line: usize,
    column: usize
}

impl Parser{
    pub fn new(tokens: Vec<Token>, label_map: HashMap<String, u16>) -> Self{
        let instruction_map: HashMap<String, u16> = HashMap::from([
            (String::from("ADD"), 0x0),
            (String::from("SUB"), 0x1),
            (String::from("AND"), 0x2),
            (String::from("OR"),  0x3),
            (String::from("XOR"), 0x4),
            (String::from("NOT"), 0x5),
            (String::from("LDX"), 0x6),
            (String::from("LDY"), 0x7),
            (String::from("STX"), 0x8),
            (String::from("STY"), 0x9),
            (String::from("JMP"), 0xA),
            (String::from("JIC"), 0xB),
            (String::from("JIZ"), 0xC),
            (String::from("HLT"), 0xD)
        ]);
        Self{
            tokens: tokens.clone(),
            current_token: tokens[0].clone(),
            label_map: label_map,
            instruction_map: instruction_map,
            program_size: 0,
            index: 0
        }
    }

    pub fn advance(&mut self){
        if let Some(token) = self.tokens.get(self.index + 1){
            self.current_token = (*token).clone();
            self.index += 1;
            return;
        }
        self.current_token = Token::new(TokenType::EndOfFile, '\0'.to_string(), self.current_token.get_line(), self.current_token.get_column());
    }

    pub fn eat(&mut self, token_type: TokenType) -> Result<(), ParserError>{
        if self.current_token.get_type() == token_type{
            self.advance();
            return Ok(());
        }
        return Err(ParserError::from_token(
            format!("Unexpected token '{}'. Expected a token type {:#?}", self.current_token.get_value(), token_type), self.current_token.clone()
        ));
    }

    pub fn parse_argument(&mut self) -> Result<u16, ParserError>{
        let token: Token = self.current_token.clone();
        match self.current_token.get_type(){
            TokenType::Identifier => {
                if let Some(_) = self.instruction_map.get(&token.get_value()){
                    return Ok(0u16);
                } 
                self.advance();
                if let Some(arg) = self.label_map.get(&token.get_value()){
                    return Ok(*arg)
                }
                return Err(ParserError::from_token(format!("No label named '{}'", token.get_value()), token));
            },
            TokenType::Number => {
                self.advance();
                if let Ok(arg) = token.get_value().parse::<u16>(){
                    return Ok(arg)
                }
                return Err(ParserError::from_token(format!("Failed to parse number '{}'. (How...)", token.get_value()), token));
            },
            TokenType::PorcentSign => {
                self.advance();
                if let Ok(arg) = u16::from_str_radix(self.current_token.get_value().as_str(), 2){
                    return Ok(arg);
                }
                return Err(ParserError::from_token(
                    format!("Expected a 16 bit or less binary number not {}", self.current_token.get_value()), self.current_token.clone()
                ));
            },
            TokenType::AndSign => {
                self.advance();
                if let Ok(arg) = u16::from_str_radix(self.current_token.get_value().as_str(), 16){
                    return Ok(arg);
                }
                return Err(ParserError::from_token(
                    format!("Expected a 16 bit or less hexadecimal number not {}", self.current_token.get_value()), self.current_token.clone())
                );
            },
            _ => {
                return Ok(0u16);
            }
        }
    }

    pub fn parse_instruction(&mut self) -> Result<Option<Instruction>, ParserError>{
        let name: String = self.current_token.get_value();
        let token: Token = self.current_token.clone();
        self.eat(TokenType::Identifier)?;
        if self.current_token.get_type() == TokenType::DoubleDot{
            self.advance();
            return Ok(None);
        }
        let is_pointer_argument: bool = self.current_token.get_type() == TokenType::DollarSign;
        if is_pointer_argument {
            self.advance();
        }
        let argument: u16 = self.parse_argument()?;
        if let Some(instruction_type) = self.instruction_map.get(&name.clone()){
            let mut instruction_type: u16 = (*instruction_type).clone();
            if is_pointer_argument{
                if instruction_type == 0x6{
                    instruction_type = 0xE;
                }else if instruction_type == 0x7{
                    instruction_type = 0xF;
                }
            }else{
                if instruction_type == 0x8 ||  instruction_type == 0x9{
                    return Err(ParserError::from_token(format!("Expected a pointer argument for STX/STY instructions"), token));
                }
            }
            self.program_size += 1;
            return Ok(Some(Instruction::new(instruction_type, argument)));
        }
        return Err(ParserError::from_token(format!("Unexpected instruction type: '{}'", name), token));
    }

    pub fn parse(&mut self) -> Result<Vec<Instruction>, ParserError>{
        let mut result: Vec<Instruction> = Vec::new();
        while self.current_token.get_type() != TokenType::EndOfFile{
            if self.current_token.get_type() == TokenType::Identifier{
                let ins: Option<Instruction> = self.parse_instruction()?;
                if let Some(instruction) = ins{
                    result.push(instruction);
                }
            }else{
                return Err(
                    ParserError::from_token(
                        format!("Unexpected token: '{}'. Expected an instruction/identifier", self.current_token.get_value()), self.current_token.clone()
                    )
                );
            }
        }
        Ok(result)
    }
}

impl ParserError{
    pub fn new(message: String, line: usize, column: usize) -> Self{
        Self{
            message: message,
            line: line,
            column: column
        }
    }

    pub fn from_token(message: String, token: Token) -> Self{
        ParserError::new(message, token.get_line(), token.get_column())
    }

    pub fn report(&self){
        println!("ParserError: {} at line {}, column: {}", self.message, self.line, self.column);
    }
}
