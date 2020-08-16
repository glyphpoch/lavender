use crate::emulator::Emulator;
use instructions::*;

/// Decodes and runs the instruction using the given emulator, and returns the
/// number of cycles used.
pub fn process_instruction(emulator: &mut Emulator, instruction: u16) -> u32 {
    decode_instruction(instruction)(emulator, instruction)
}

pub fn decode_instruction(instruction: u16) -> fn(&mut Emulator, u16) -> u32 {
    let category = instruction >> 13 & 7;

    match category {
        0b000 => {
            // Shift by rotate
            let opcode = instruction >> 10 & 0x3;
            match opcode {
                0b00 => lsl,
                0b01 => lsr,
                0b10 => asr,
                0b11 => {
                    let opc = instruction >> 8 & 0x3;
                    match opc {
                        0b00 => add, // ADD(3) add register
                        0b01 => sub, // SUB(3) sub register
                        0b10 => add, // ADD(1) add immediate
                        0b11 => sub, // SUB(1) sub immediate
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        0b001 => {
            // Add/subtract/compare/move immediate
            let opcode = instruction >> 10 & 0x3;
            match opcode {
                0b00 => mov, // MOV(1) mov large immediate
                0b01 => cmp, // CMP(1) cmp large immediate
                0b10 => add, // ADD(2) add large immediate
                0b11 => sub, // SUB(2) sub large immediate
                _ => unreachable!(),
            }
        }
        0b010 => {
            let subcategory = instruction >> 10 & 0x7;
            match subcategory {
                0b000 => {
                    // Data processing register
                    let opcode = instruction >> 6 & 0xf;

                    match opcode {
                        0b0000 => and,
                        0b0001 => eor,
                        0b0010 => lsl, // LSL(2)
                        0b0011 => lsr, // LSR(2)
                        0b0100 => asr, // ASR(2)
                        0b0101 => adc,
                        0b0110 => sbc,
                        0b0111 => ror,
                        0b1000 => tst,
                        0b1001 => neg,
                        0b1010 => cmp, // CMP(2) cmp registers
                        0b1011 => cmn,
                        0b1100 => or,
                        0b1101 => mul,
                        0b1110 => bic,
                        0b1111 => mvn,
                        _ => unreachable!(),
                    }
                }
                0b001 => {
                    // Special data processing and branch/exchange
                    let opcode = instruction >> 8 & 0x3;
                    match opcode {
                        0b00 => add, // ADD(4)
                        0b01 => cmp, // CMP(3)
                        //0b10 => cpy,
                        0b11 => bx, // if the bit after is 0b1, then this is a blx (ARMv5 and higher)
                        _ => unreachable!(),
                    }
                }
                0b010 | 0b011 => ldr, // LDR(3) Load from literal pool
                0b100..=0b111 => {
                    // Load/store register offset
                    let opcode = instruction >> 9 & 0x7;
                    match opcode {
                        0b000 => str,  // STR(2)
                        0b001 => strh, // STRH(2)
                        0b010 => strb, // STRB(2)
                        0b011 => ldrsb,
                        0b100 => ldr,  // LDR(2)
                        0b101 => ldrh, // LDRH(2)
                        0b110 => ldrb, // LDRB(2)
                        0b111 => ldrsh,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        0b011 => {
            // Load/store word/byte immediate offset
            let b = instruction >> 12 & 0x1 > 0;
            let l = instruction >> 11 & 0x1 > 0;
            match (b, l) {
                (false, false) => str, // STR(1)
                (false, true) => ldr,  // LDR(1)
                (true, false) => strb, // STRB(1)
                (true, true) => ldrb,  // LDRB(1)
            }
        }
        0b100 => {
            let stack = instruction >> 12 & 1 > 0;
            match stack {
                true => {
                    // Load/store to/from stack
                    let l = instruction >> 11 & 0x1 > 0;
                    match l {
                        false => str, // STR(3)
                        true => ldr,  // LDR(4)
                    }
                }
                false => {
                    // Load/store halfword immediate offset
                    let l = instruction >> 11 & 0x1 > 0;
                    match l {
                        false => strh, // STRH(1)
                        true => ldrh,  // LDRH(1)
                    }
                }
            }
        }
        0b101 => {
            let misc = instruction >> 12 & 1 > 0;
            match misc {
                true => {
                    // miscellaneous instructions
                    let misc_code = instruction >> 7 & 0x1f;
                    match misc_code {
                        0b00000 => add, // ADD(7)
                        0b00001 => sub, // SUB(4)
                        0b01000..=0b11011 => {
                            // TODO: Dangerous match
                            let l = instruction >> 1 & 0x1 > 0;
                            match l {
                                false => push,
                                true => pop,
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                false => {
                    // Add to SP or PC immediate
                    let sp = instruction >> 11 & 0x1 > 0;
                    match sp {
                        false => add, // ADD(5)
                        true => add,  // ADD(6)
                    }
                }
            }
        }
        0b110 => {
            let branch = instruction >> 12 & 1 > 0;
            let condition = instruction >> 8 & 0xf;
            match (branch, condition) {
                (true, 0b1110) => unreachable!(), // undefined
                (true, 0b1111) => swi,            // swi
                (true, _) => b,                   // B(1) conditional branch things
                (false, _) => {
                    // load/store multiple
                    let l = instruction >> 11 & 0x1 > 0;
                    match l {
                        false => stmia,
                        true => ldmia,
                    }
                }
            }
        }
        0b111 => b, // B(2) unconditional branches
        _ => unreachable!(),
    }
}

pub fn placeholder(_emulator: &mut Emulator, _instruction: u16) -> u32 {
    1
}

pub mod instructions {
    use super::super::arm::instructions::*;
    use crate::emulator::Emulator;

    pub fn adc(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn add(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn and(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn asr(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn b(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn bic(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn bl(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn bx(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn cmn(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn cmp(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    /// Logical XOR
    pub fn eor(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn ldmia(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn ldr(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn ldrb(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn ldrh(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn ldrsb(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn ldrsh(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn lsl(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn lsr(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn mov(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn mul(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn mvn(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn neg(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    /// Logical OR (also referred to as the orr instruction)
    pub fn or(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn pop(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn push(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn ror(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn sbc(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn stmia(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn str(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn strb(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn strh(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn sub(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn swi(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn tst(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
}
