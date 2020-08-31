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
                0b00 => lsl1,
                0b01 => lsr1,
                0b10 => asr1,
                0b11 => {
                    let opc = instruction >> 8 & 0x3;
                    match opc {
                        0b00 => add3,
                        0b01 => sub3,
                        0b10 => add1,
                        0b11 => sub1,
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
                0b00 => mov1,
                0b01 => cmp1,
                0b10 => add2,
                0b11 => sub2,
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
                        0b0010 => lsl2,
                        0b0011 => lsr2,
                        0b0100 => asr2,
                        0b0101 => adc,
                        0b0110 => sbc,
                        0b0111 => ror,
                        0b1000 => tst,
                        0b1001 => neg,
                        0b1010 => cmp2,
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
                        0b00 => add4,
                        0b01 => cmp3,
                        0b10 => mov3,
                        0b11 => bx, // if the bit after is 0b1, then this is a blx (ARMv5 and higher)
                        _ => unreachable!(),
                    }
                }
                0b010 | 0b011 => ldr3, // LDR(3) Load from literal pool
                0b100..=0b111 => {
                    // Load/store register offset
                    let opcode = instruction >> 9 & 0x7;
                    match opcode {
                        0b000 => str2,
                        0b001 => strh2,
                        0b010 => strb2,
                        0b011 => ldrsb,
                        0b100 => ldr2,
                        0b101 => ldrh2,
                        0b110 => ldrb2,
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
                (false, false) => str1,
                (false, true) => ldr1,
                (true, false) => strb1,
                (true, true) => ldrb1,
            }
        }
        0b100 => {
            let stack = instruction >> 12 & 1 > 0;
            match stack {
                true => {
                    // Load/store to/from stack
                    let l = instruction >> 11 & 0x1 > 0;
                    match l {
                        false => str3,
                        true => ldr4,
                    }
                }
                false => {
                    // Load/store halfword immediate offset
                    let l = instruction >> 11 & 0x1 > 0;
                    match l {
                        false => strh1,
                        true => ldrh1,
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
                        0b00000 => add7,
                        0b00001 => sub4,
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
                        false => add5,
                        true => add6,
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
                (true, _) => b1,                  // B(1) conditional branch things
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
        0b111 => b2, // B(2) unconditional branches
        _ => unreachable!(),
    }
}

#[macro_use]
mod internal {
    use crate::emulator::{
        armv4t::utils::*,
        cpu::RegisterNames::{self, *},
        Emulator,
    };
    use std::convert::TryFrom;

    macro_rules! instruction_format_1 {
        ($emulator:expr, $instruction:expr, $operation:expr, $carry_fn:expr, $overflow_fn:expr) => {{
            let instruction = $instruction as u32;
            let destination_register = RegisterNames::try_from(instruction & 0x7).unwrap();
            let first_operand_register = RegisterNames::try_from(instruction >> 3 & 0x7).unwrap();
            let second_operand_register = RegisterNames::try_from(instruction >> 6 & 0x7).unwrap();

            let first_operand = $emulator.cpu.get_register_value(first_operand_register);
            let second_operand = $emulator.cpu.get_register_value(second_operand_register);

            let result = $operation(first_operand, second_operand);

            $emulator
                .cpu
                .set_register_value(destination_register, result);

            $emulator.cpu.set_nzcv(
                result.is_bit_set(31),
                result == 0,
                $carry_fn(first_operand, second_operand),
                $overflow_fn(first_operand, second_operand, result),
            );
        }};
    }

    macro_rules! instruction_format_2 {
        ($emulator:expr, $instruction:expr, $operation:expr, $carry_fn:expr, $overflow_fn:expr) => {{
            let instruction = $instruction as u32;
            let destination_register = RegisterNames::try_from(instruction & 0x7).unwrap();
            let first_operand_register = RegisterNames::try_from(instruction >> 3 & 0x7).unwrap();
            let immed_3 = instruction >> 6 & 0x7;

            let first_operand = $emulator.cpu.get_register_value(first_operand_register);

            let result = $operation(first_operand, immed_3);

            $emulator
                .cpu
                .set_register_value(destination_register, result);

            $emulator.cpu.set_nzcv(
                result.is_bit_set(31),
                result == 0,
                $carry_fn(first_operand, immed_3),
                $overflow_fn(first_operand, immed_3, result),
            );
        }};
    }

    macro_rules! instruction_format_3 {
        ($emulator:expr, $instruction:expr, $operation:expr, $carry_fn:expr, $overflow_fn:expr) => {{
            let instruction = $instruction as u32;
            let destination_register = RegisterNames::try_from(instruction >> 8 & 0x7).unwrap();
            let first_operand = $emulator.cpu.get_register_value(destination_register);
            let immed_8 = instruction & 0xff;

            let result = $operation($emulator, destination_register, first_operand, immed_8);

            $emulator.cpu.set_nzcv(
                result.is_bit_set(31),
                result == 0,
                $carry_fn(first_operand, immed_8),
                $overflow_fn(first_operand, immed_8, result),
            );
        }};

        // For CMP and MOV
        ($emulator:expr, $instruction:expr, $operation:expr) => {{
            let instruction = $instruction as u32;
            let destination_register = RegisterNames::try_from(instruction >> 8 & 0x7).unwrap();
            let first_operand = $emulator.cpu.get_register_value(destination_register);
            let immed_8 = instruction & 0xff;

            let result = $operation($emulator, destination_register, first_operand, immed_8);

            $emulator.cpu.set_nzcv(
                result.is_bit_set(31),
                result == 0,
                $emulator.cpu.get_c(),
                $emulator.cpu.get_v(),
            );
        }};
    }

    macro_rules! instruction_format_4 {
        ($emulator:expr, $instruction:expr, $operation:expr) => {{
            let instruction = $instruction as u32;
            let destination_register = RegisterNames::try_from(instruction & 0x7).unwrap();
            let first_operand_register = RegisterNames::try_from(instruction >> 3 & 0x7).unwrap();
            let immed_5 = instruction >> 6 & 0x1f;

            let first_operand = $emulator.cpu.get_register_value(first_operand_register);

            let (carry_flag, result) = $operation($emulator, first_operand, immed_5);

            $emulator
                .cpu
                .set_register_value(destination_register, result);

            $emulator.cpu.set_nzcv(
                result.is_bit_set(31),
                result == 0,
                carry_flag,
                $emulator.cpu.get_v(),
            );
        }};
    }

    macro_rules! instruction_format_5 {
        ($emulator:expr, $instruction:expr, $operation:expr) => {{
            let instruction = $instruction as u32;
            let destination_register = RegisterNames::try_from(instruction & 0x7).unwrap();
            let second_operand_register = RegisterNames::try_from(instruction >> 3 & 0x7).unwrap();

            let first_operand = $emulator.cpu.get_register_value(destination_register);
            let second_operand = $emulator.cpu.get_register_value(second_operand_register);

            let (carry_flag, overflow_flag, result) = $operation(
                $emulator,
                destination_register,
                first_operand,
                second_operand,
            );

            $emulator.cpu.set_nzcv(
                result.is_bit_set(31),
                result == 0,
                carry_flag,
                overflow_flag,
            );
        }};
    }

    macro_rules! instruction_format_6 {
        ($emulator:expr, $instruction:expr, $operation:expr, $operation_register:expr) => {{
            let instruction = $instruction as u32;
            let destination_register = RegisterNames::try_from(instruction & 0x7).unwrap();
            let immed_8 = instruction & 0xff;

            let first_operand = $emulator.cpu.get_register_value($operation_register);

            let result = $operation(first_operand, immed_8);

            $emulator
                .cpu
                .set_register_value(destination_register, result);
        }};
    }

    macro_rules! instruction_format_7 {
        ($emulator:expr, $instruction:expr, $operation:expr) => {{
            let instruction = $instruction as u32;

            let sp = r13;

            let immed_7 = instruction & 0x7f;
            let first_operand = $emulator.cpu.get_register_value(sp);

            let result = $operation(first_operand, immed_7 << 2);

            $emulator.cpu.set_register_value(sp, result);
        }};
    }

    macro_rules! instruction_format_8 {
        ($emulator:expr, $instruction:expr, $operation:expr, $flags_operation:expr) => {{
            let instruction = $instruction as u32;

            let high_bit_1 = instruction >> 7 & 0x1;
            let high_bit_2 = instruction >> 6 & 0x1;

            let first_operand_register = RegisterNames::try_from(high_bit_1 << 3 | instruction & 0x7).unwrap();
            let second_operand_register = RegisterNames::try_from(high_bit_2 << 3 | instruction >> 3 & 0x7).unwrap();

            // TODO: UNPREDICTABLE if first_operand_register == r15
            // TODO: If a low register is specified for both <Rn> and <Rm> (H1==0 and H2==0), the result is UNPREDICTABLE.

            let first_operand = $emulator.cpu.get_register_value(first_operand_register);
            let second_operand = $emulator.cpu.get_register_value(second_operand_register);

            let (carry_flag, overflow_flag, result) = $operation(
                $emulator,
                first_operand_register,
                first_operand,
                second_operand,
            );

            $flags_operation($emulator, result, carry_flag, overflow_flag);
        }};

        ($emulator:expr, $instruction:expr, $operation:expr) => {{
            let instruction = $instruction as u32;

            let high_bit_1 = instruction >> 7 & 0x1;
            let high_bit_2 = instruction >> 6 & 0x1;

            let first_operand_register = RegisterNames::try_from(high_bit_1 << 3 | instruction & 0x7).unwrap();
            let second_operand_register = RegisterNames::try_from(high_bit_2 << 3 | instruction >> 3 & 0x7).unwrap();

            // TODO: UNPREDICTABLE if first_operand_register == r15
            // TODO: If a low register is specified for both <Rn> and <Rm> (H1==0 and H2==0), the result is UNPREDICTABLE.

            let first_operand = $emulator.cpu.get_register_value(first_operand_register);
            let second_operand = $emulator.cpu.get_register_value(second_operand_register);

            let result = $operation(
                first_operand,
                second_operand,
            );

            $emulator.cpu.set_register_value(first_operand_register, result);
        }};
    }
}
pub mod instructions {
    use super::super::arm::instructions::*;
    use crate::emulator::{
        armv4t::utils::*,
        cpu::RegisterNames::{self, *},
        Emulator,
    };
    use std::convert::TryFrom;

    /// Add with Carry
    pub fn adc(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_5!(
            emulator,
            instruction,
            |emulator: &mut Emulator, destination_register, first_operand: u32, second_operand| {
                let carry_amount = if emulator.cpu.get_c() { 1 } else { 0 };

                let result = first_operand
                    .wrapping_add(second_operand)
                    .wrapping_add(carry_amount);

                (
                    carry_from_with_carry(first_operand, second_operand, carry_amount),
                    addition_overflow(first_operand, second_operand, result),
                    result,
                )
            }
        );

        1
    }

    /// Addition (adds a 3-bit integer to a value of a register)
    pub fn add1(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_2!(
            emulator,
            instruction,
            u32::wrapping_add,
            carry_from,
            addition_overflow
        );

        1
    }

    /// Add a large immediate value to the value of a register
    pub fn add2(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_3!(
            emulator,
            instruction,
            |emulator: &mut Emulator, destination_register, first_operand: u32, immed_8| -> u32 {
                let result = first_operand.wrapping_add(immed_8);
                emulator
                    .cpu
                    .set_register_value(destination_register, result);
                result
            },
            carry_from,
            addition_overflow
        );

        1
    }

    /// Addition (adds values of two registers)
    pub fn add3(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_1!(
            emulator,
            instruction,
            u32::wrapping_add,
            carry_from,
            addition_overflow
        );

        1
    }

    /// Adds the values of two registers, one or both of which are high registers
    pub fn add4(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_8!(emulator, instruction, |first_operand: u32,
                                                      second_operand|
         -> u32 {
            first_operand.wrapping_add(second_operand)
        });

        1
    }

    /// Adds an immediate value to the PC
    pub fn add5(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_6!(
            emulator,
            instruction,
            |first_operand: u32, immed_8| -> u32 {
                (first_operand & 0xFFFF_FFFC).wrapping_add(immed_8 << 2)
            },
            r15
        );

        1
    }

    /// Adds an immediate value to the SP
    pub fn add6(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_6!(
            emulator,
            instruction,
            |first_operand: u32, immed_8| -> u32 { first_operand.wrapping_add(immed_8 << 2) },
            r13
        );

        1
    }

    /// Increments the SP by four times a 7-bit immediate
    pub fn add7(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_7!(emulator, instruction, u32::wrapping_add);

        1
    }

    /// Logical AND
    pub fn and(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_5!(
            emulator,
            instruction,
            |emulator: &mut Emulator, destination_register, first_operand: u32, second_operand| {
                let result = first_operand & second_operand;

                emulator
                    .cpu
                    .set_register_value(destination_register, result);

                (emulator.cpu.get_c(), emulator.cpu.get_v(), result)
            }
        );

        1
    }

    /// Arithmetic Shift Right
    pub fn asr1(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_4!(emulator, instruction, |_: &mut Emulator,
                                                      first_operand: u32,
                                                      immed_5|
         -> (bool, u32) {
            if immed_5 == 0 {
                let first_operand_positive = first_operand.is_bit_set(31);
                if first_operand_positive {
                    (false, 0)
                } else {
                    (true, 0xFFFF_FFFF)
                }
            } else {
                (
                    first_operand.is_bit_set(immed_5 - 1),
                    (first_operand as i32 >> immed_5) as u32,
                )
            }
        });

        1
    }

    /// Arithmetic Shift Right
    pub fn asr2(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_5!(
            emulator,
            instruction,
            |emulator: &mut Emulator, destination_register, first_operand: u32, second_operand| {
                let second_operand = second_operand & 0xff;

                let (carry_flag, result) = if second_operand == 0 {
                    (emulator.cpu.get_c(), first_operand)
                } else if second_operand < 32 {
                    (
                        first_operand.is_bit_set(second_operand - 1),
                        (first_operand as i32).wrapping_shr(second_operand) as u32,
                    )
                } else {
                    (
                        first_operand.is_bit_set(31),
                        if first_operand == 0 { 0 } else { 0xFFFF_FFFF },
                    )
                };

                emulator
                    .cpu
                    .set_register_value(destination_register, result);

                (carry_flag, emulator.cpu.get_v(), result)
            }
        );

        1
    }

    pub fn b1(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn b2(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }

    /// Bit Clear
    pub fn bic(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_5!(
            emulator,
            instruction,
            |emulator: &mut Emulator, destination_register, first_operand, second_operand: u32| {
                let result = first_operand & !second_operand;

                emulator
                    .cpu
                    .set_register_value(destination_register, result);

                (emulator.cpu.get_c(), emulator.cpu.get_v(), result)
            }
        );

        1
    }

    pub fn bl(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn bx(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }

    /// Compare negative
    pub fn cmn(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_5!(
            emulator,
            instruction,
            |emulator: &mut Emulator, _, first_operand: u32, second_operand| {
                let alu_out = first_operand.wrapping_add(second_operand);
                (
                    carry_from(first_operand, second_operand),
                    addition_overflow(first_operand, second_operand, alu_out),
                    alu_out,
                )
            }
        );

        1
    }

    /// Compare (a register value with a large immediate value)
    pub fn cmp1(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_3!(
            emulator,
            instruction,
            |emulator: &mut Emulator, destination_register, first_operand: u32, immed_8| -> u32 {
                first_operand.wrapping_sub(immed_8)
            },
            not_borrow_from,
            substraction_overflow
        );

        1
    }

    /// Compare two register values
    pub fn cmp2(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_5!(
            emulator,
            instruction,
            |emulator: &mut Emulator, _, first_operand: u32, second_operand| {
                let alu_out = first_operand.wrapping_sub(second_operand);
                (
                    not_borrow_from(first_operand, second_operand),
                    substraction_overflow(first_operand, second_operand, alu_out),
                    alu_out,
                )
            }
        );

        1
    }

    /// Compare the values of two registers (one or both can be high registers)
    pub fn cmp3(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_8!(
            emulator,
            instruction,
            |emulator: &mut Emulator, _, first_operand: u32, second_operand| -> (bool, bool, u32) {
                let result = first_operand.wrapping_sub(second_operand);
                (
                    not_borrow_from(first_operand, second_operand),
                    substraction_overflow(first_operand, second_operand, result),
                    result,
                )
            },
            |emulator: &mut Emulator, result: u32, carry_flag, overflow_flag| {
                emulator.cpu.set_nzcv(
                    result.is_bit_set(31),
                    result == 0,
                    carry_flag,
                    overflow_flag,
                );
            }
        );

        1
    }

    /// Exclusive OR
    pub fn eor(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_5!(
            emulator,
            instruction,
            |emulator: &mut Emulator, destination_register, first_operand: u32, second_operand| {
                let result = first_operand ^ second_operand;

                emulator
                    .cpu
                    .set_register_value(destination_register, result);

                (emulator.cpu.get_c(), emulator.cpu.get_v(), result)
            }
        );

        1
    }

    pub fn ldmia(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn ldr1(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn ldr2(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn ldr3(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn ldr4(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn ldrb1(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn ldrb2(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn ldrh1(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn ldrh2(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn ldrsb(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn ldrsh(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }

    /// Logical Shift Left
    pub fn lsl1(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_4!(emulator, instruction, |emulator: &mut Emulator,
                                                      first_operand: u32,
                                                      immed_5|
         -> (bool, u32) {
            if immed_5 == 0 {
                (emulator.cpu.get_c(), first_operand)
            } else {
                (
                    first_operand.is_bit_set(32 - immed_5),
                    first_operand << immed_5,
                )
            }
        });

        1
    }

    /// Logical Shift Left
    pub fn lsl2(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_5!(
            emulator,
            instruction,
            |emulator: &mut Emulator, destination_register, first_operand: u32, second_operand| {
                let second_operand = second_operand & 0xff;

                let (carry_flag, result) = if second_operand == 0 {
                    (emulator.cpu.get_c(), first_operand)
                } else if second_operand < 32 {
                    (
                        first_operand.is_bit_set(32 - second_operand),
                        first_operand << second_operand,
                    )
                } else if second_operand == 32 {
                    (first_operand.is_bit_set(0), 0)
                } else {
                    (false, 0)
                };

                emulator
                    .cpu
                    .set_register_value(destination_register, result);

                (carry_flag, emulator.cpu.get_v(), result)
            }
        );

        1
    }

    /// Logical Shift Right
    pub fn lsr1(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_4!(emulator, instruction, |_: &mut Emulator,
                                                      first_operand: u32,
                                                      immed_5|
         -> (bool, u32) {
            if immed_5 == 0 {
                (first_operand.is_bit_set(31), 0)
            } else {
                (
                    first_operand.is_bit_set(immed_5 - 1),
                    first_operand >> immed_5,
                )
            }
        });

        1
    }

    /// Logical Shift Right
    pub fn lsr2(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_5!(
            emulator,
            instruction,
            |emulator: &mut Emulator, destination_register, first_operand: u32, second_operand| {
                let second_operand = second_operand & 0xff;

                let (carry_flag, result) = if second_operand == 0 {
                    (emulator.cpu.get_c(), first_operand)
                } else if second_operand < 32 {
                    (
                        first_operand.is_bit_set(second_operand - 1),
                        first_operand >> second_operand,
                    )
                } else if second_operand == 32 {
                    (first_operand.is_bit_set(31), 0)
                } else {
                    (false, 0)
                };

                emulator
                    .cpu
                    .set_register_value(destination_register, result);

                (carry_flag, emulator.cpu.get_v(), result)
            }
        );

        1
    }

    /// Move a large immediate value to a register
    pub fn mov1(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_3!(emulator, instruction, |emulator: &mut Emulator,
                                                      destination_register,
                                                      _,
                                                      immed_8|
         -> u32 {
            emulator
                .cpu
                .set_register_value(destination_register, immed_8);
            immed_8
        });

        1
    }

    pub fn mov2(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }

    /// Moves a value to, from, or between high registers
    pub fn mov3(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_8!(emulator, instruction, |_, second_operand: u32| -> u32 {
            second_operand
        });

        1
    }

    /// Multiply
    pub fn mul(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_5!(
            emulator,
            instruction,
            |emulator: &mut Emulator, destination_register, first_operand: u32, second_operand| {
                let result = first_operand.wrapping_mul(second_operand);

                emulator
                    .cpu
                    .set_register_value(destination_register, result);

                (emulator.cpu.get_c(), emulator.cpu.get_v(), result)
            }
        );

        1
    }

    /// Move NOT
    pub fn mvn(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_5!(
            emulator,
            instruction,
            |emulator: &mut Emulator, destination_register, _, second_operand: u32| {
                let result = !second_operand;
                emulator
                    .cpu
                    .set_register_value(destination_register, result);
                (emulator.cpu.get_c(), emulator.cpu.get_v(), result)
            }
        );

        1
    }

    /// Negate
    pub fn neg(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_5!(
            emulator,
            instruction,
            |emulator: &mut Emulator, destination_register, _, second_operand| {
                let result = (0 as u32).wrapping_sub(second_operand);

                emulator
                    .cpu
                    .set_register_value(destination_register, result);

                (
                    not_borrow_from(0, second_operand),
                    substraction_overflow(0, second_operand, result),
                    result,
                )
            }
        );

        1
    }

    /// Logical OR (also referred to as the orr instruction)
    pub fn or(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_5!(
            emulator,
            instruction,
            |emulator: &mut Emulator, destination_register, first_operand: u32, second_operand| {
                let result = first_operand | second_operand;

                emulator
                    .cpu
                    .set_register_value(destination_register, result);

                (emulator.cpu.get_c(), emulator.cpu.get_v(), result)
            }
        );

        1
    }

    pub fn pop(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn push(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }

    /// Rotate Right Register
    pub fn ror(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_5!(
            emulator,
            instruction,
            |emulator: &mut Emulator, destination_register, first_operand: u32, second_operand| {
                let second_operand = second_operand & 0xff;
                let second_operand_5_bit = second_operand & 0x1f;

                let (carry_flag, result) = if second_operand == 0 {
                    (emulator.cpu.get_c(), first_operand)
                } else if second_operand_5_bit == 0 {
                    (first_operand.is_bit_set(31), first_operand)
                } else {
                    (
                        first_operand.is_bit_set(second_operand_5_bit - 1),
                        first_operand.rotate_right(second_operand_5_bit),
                    )
                };

                emulator
                    .cpu
                    .set_register_value(destination_register, result);

                (carry_flag, emulator.cpu.get_v(), result)
            }
        );

        1
    }

    /// Substract with Carry
    pub fn sbc(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_5!(
            emulator,
            instruction,
            |emulator: &mut Emulator, destination_register, first_operand: u32, second_operand| {
                let carry_amount = if !emulator.cpu.get_c() { 1 } else { 0 };

                let result = first_operand
                    .wrapping_sub(second_operand)
                    .wrapping_sub(carry_amount);

                // TODO: Maybe these shouldn't be bool's, too easy to mess up
                (
                    not_borrow_from_with_carry(first_operand, second_operand, carry_amount),
                    substraction_overflow(first_operand, second_operand, result),
                    result,
                )
            }
        );

        1
    }

    pub fn stmia(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn str1(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn str2(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn str3(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn strb1(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn strb2(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn strh1(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }
    pub fn strh2(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }

    /// Substraction (substracts a 3-bit integer from the value of a register)
    pub fn sub1(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_2!(
            emulator,
            instruction,
            u32::wrapping_sub,
            not_borrow_from,
            substraction_overflow
        );

        1
    }

    /// Substract a large immediate value from the value of a register
    pub fn sub2(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_3!(
            emulator,
            instruction,
            |emulator: &mut Emulator, destination_register, first_operand: u32, immed_8| -> u32 {
                let result = first_operand.wrapping_sub(immed_8);
                emulator
                    .cpu
                    .set_register_value(destination_register, result);
                result
            },
            not_borrow_from,
            substraction_overflow
        );

        1
    }

    /// Substraction (substracts values of two registers)
    pub fn sub3(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_1!(
            emulator,
            instruction,
            u32::wrapping_sub,
            not_borrow_from,
            substraction_overflow
        );

        1
    }

    /// Decrements the SP by four rimes a 7-bit immediate
    pub fn sub4(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_7!(emulator, instruction, u32::wrapping_sub);

        1
    }

    pub fn swi(_emulator: &mut Emulator, _instruction: u16) -> u32 {
        1
    }

    /// Test
    pub fn tst(emulator: &mut Emulator, instruction: u16) -> u32 {
        instruction_format_5!(
            emulator,
            instruction,
            |emulator: &mut Emulator, destination_register, first_operand, second_operand| {
                let alu_out = first_operand & second_operand;
                (emulator.cpu.get_c(), emulator.cpu.get_v(), alu_out)
            }
        );

        1
    }
}
