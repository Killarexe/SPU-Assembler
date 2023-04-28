#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum TokenType{
    Identifier,
    Number,
    DollarSign,
    PorcentSign,
    AndSign,
    DoubleDot,
    EndOfFile
}

#[derive(Clone)]
pub struct Token{
    token_type: TokenType,
    value: String,
    line: usize,
    column: usize
}

impl Token{
    pub fn new(token_type: TokenType, value: String, line: usize, column: usize) -> Self{
        Self{
            token_type: token_type,
            value: value,
            line: line,
            column: column
        }
    }

    pub fn get_type(&self) -> TokenType{
        self.token_type.clone()
    }

    pub fn get_value(&self) -> String{
        self.value.clone()
    }

    pub fn get_line(&self) -> usize{
        self.line.clone()
    }

    pub fn get_column(&self) -> usize{
        self.column.clone()
    }

    /*pub fn as_string(&self) -> String{
        format!("Token[type: {:#?}, value: {}, line: {}, column: {}]", self.token_type, self.value, self.line, self.column)
    }*/
}
