pub struct Instruction{
    instruction_type: u16,
    argument: u16,
}

impl Instruction{
    pub fn new(instruction_type: u16, argument: u16) -> Self{
        Self{
            instruction_type: instruction_type,
            argument: argument
        }
    }

    pub fn get_as_string(&self) -> String{
        let mut instruction_name: String = String::from("UKN");
        match self.instruction_type{
            0 => instruction_name = String::from("ADD"),
            1 => instruction_name = String::from("SUB"),
            2 => instruction_name = String::from("AND"),
            3 => instruction_name = String::from("OR "),
            4 => instruction_name = String::from("XOR"),
            5 => instruction_name = String::from("NOT"),
            6 | 14 => instruction_name = String::from("LDX"),
            7 | 15 => instruction_name = String::from("LDY"),
            8 => instruction_name = String::from("STX"),
            9 => instruction_name = String::from("STY"),
            10 => instruction_name = String::from("JMP"),
            11 => instruction_name = String::from("JIC"),
            12 => instruction_name = String::from("JIZ"),
            13 => instruction_name = String::from("HLT"),
            _ => {}
        }
        format!("{} {}", instruction_name, self.argument)
    }

    pub fn as_u16(&self) -> u16{
        self.argument + ((self.instruction_type as u16) << 12)
    }
}
