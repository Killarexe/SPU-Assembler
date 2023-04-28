# SPU Assembler

## About the SPU

SPU(Simple Proccessing Unit) is a 16-Bit Proccessor with simple instructions and 2 registers.
It's goal is to introduce easly to the _beautiful_ world of hardware and binary.

## SPU Instruction Set

| Instruction | Argument   | What it does                                        |
| ----------- | ---------- | --------------------------------------------------- |
| ADD         | None       | Do X + Y and return the value to X.                 |
| SUB         | None       | Do X - Y and return the value to X.                 |
| AND         | None       | Do X & Y and return the value to X.                 |
| OR          | None       | Do X or Y and return the value to X.                |
| XOR         | None       | Do X ^ Y and return the value to X.                 |
| NOT         | None       | Inverse the X register.                             |
| LDX         | im12/adr12 | Change the value of X.                              |
| LDY         | im12/adr12 | Change the value of Y.                              |
| STX         | adr12      | Store the X register to the ram address.            |
| STY         | adr12      | Store the Y register to the ram address.            |
| JMP         | adr12      | Jump to the label/address.                          |
| JIC         | adr12      | Jump to the label/address if the carry flag is set. |
| JIZ         | adr12      | Jump to the label/address if the zero flag is set.  |
| HLT         | None       | Stops the proccessor.                               |
