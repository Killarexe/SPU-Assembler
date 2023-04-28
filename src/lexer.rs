use crate::token::*;
use std::collections::HashMap;

pub struct Lexer{
    source: Vec<char>,
    label_map: HashMap<String, u16>,
    program_line: u16,
    current_char: char,
    line: usize,
    column: usize,
    index: usize
}

impl Lexer{
    pub fn new(source: String) -> Self{
        let src: Vec<char> = source.to_uppercase().chars().collect();
        Self{
            source: src.clone(),
            current_char: src[0],
            label_map: HashMap::new(),
            program_line: 0,
            line: 1,
            column: 1,
            index: 0
        } 
    }

    pub fn is_end_of_file(&self) -> bool{
        self.index >= self.source.len() || self.current_char == '\0'
    }

    pub fn is_current_char_alphabetic(&self) -> bool{
        (self.current_char.is_alphabetic() || self.current_char == '_' || self.current_char == '-') && !self.is_end_of_file()
    }

    pub fn advance(&mut self){
        if !self.is_end_of_file(){
            if let Some(ch) = self.source.get(self.index + 1){
                if *ch == '\n' && self.current_char != '\n'{
                    self.program_line += 1;
                }
                self.current_char = *ch;
                self.index += 1;
                self.column += 1;
                return;
            }
        }
        self.current_char = '\0'
    }

    pub fn new_token(&self, token_type: TokenType, value: String) -> Token{
       Token::new(token_type, value, self.line.clone(), self.column.clone()) 
    }

    pub fn new_token_char(&mut self, token_type: TokenType) -> Token{
        let token: Token = self.new_token(token_type, self.current_char.clone().to_string());
        self.advance();
        token
    }

    pub fn scan_number(&mut self) -> Token{
        let mut value: String = String::new();
        while self.current_char.is_numeric() && !self.is_end_of_file(){
            value.push(self.current_char.clone());
            self.advance();
        }
        self.new_token(TokenType::Number, value)
    }

    pub fn scan_identifier(&mut self) -> Token{
        let mut value: String = String::new();
        while self.is_current_char_alphabetic(){
            value.push(self.current_char.clone());
            self.advance();
        }
        if self.current_char == ':'{
            self.label_map.insert(value.clone(), self.program_line.clone());
        }
        self.new_token(TokenType::Identifier, value)
    }

    pub fn skip_whitespace(&mut self){
        while self.current_char.is_whitespace(){
            if self.current_char == '\n'{
                self.line += 1;
                self.column = 0;
            }
            self.advance();
        }
    }
    
    pub fn scan(&mut self) -> Result<Vec<Token>, LexerError>{
        let mut result: Vec<Token> = Vec::new();
        while !self.is_end_of_file(){
            self.skip_whitespace();
            match self.current_char.clone(){
                ':' => result.push(self.new_token_char(TokenType::DoubleDot)),
                '%' => result.push(self.new_token_char(TokenType::PorcentSign)),
                '&' => result.push(self.new_token_char(TokenType::AndSign)),
                '$' => result.push(self.new_token_char(TokenType::DollarSign)),
                ch => {
                    if self.is_current_char_alphabetic(){
                        result.push(self.scan_identifier());
                    }else if self.current_char.is_numeric(){
                        result.push(self.scan_number());
                    }else{
                        return Err(LexerError::new(format!("Unexpected character: '{}'", ch), self.line.clone(), self.column.clone()))
                    }
                }
            }
            self.skip_whitespace();
        }
        result.push(self.new_token(TokenType::EndOfFile, '\0'.to_string()));
        Ok(result)
    }

    pub fn get_label_map(&self) -> HashMap<String, u16>{
        return self.label_map.clone();
    }
}

pub struct LexerError{
    message: String,
    line: usize,
    column: usize
}

impl LexerError{
    pub fn new(message: String, line: usize, column: usize) -> Self{
        Self{
            message: message,
            line: line,
            column: column
        }
    }

    pub fn report(&self){
        println!("LexerError: {} at line {}, column {}", self.message, self.line, self.column);
    }
}
