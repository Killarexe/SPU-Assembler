#[derive(Copy, PartialEq, Clone, Debug)]
pub enum InstructionType{
    Add,
    Sub,
    And,
    Or,
    Xor,
    Not,
    Ldx,
    Ldy,
    Stx,
    Sty,
    Jmp,
    Jic,
    Jiz,
    Hlt,
    Ldxp,
    Ldyp
}

pub struct Instruction{
    instruction_type: InstructionType,
    argument: u16,
}

impl Instruction{
    pub fn new(instruction_type: InstructionType, argument: u16) -> Self{
        Self{
            instruction_type: instruction_type,
            argument: argument
        }
    }

    pub fn get_as_string(&self) -> String{
        format!("{:#?} {}", self.instruction_type, self.argument)
    }

    pub fn as_u16(&self) -> u16{
        self.argument + ((self.instruction_type as u16) << 12)
    }
}
