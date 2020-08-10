use crate::emulator::{cpu::*, Emulator};
use num_enum::TryFromPrimitive;
use std::cmp::Ordering;
use std::convert::TryFrom;

pub fn process_shifter_operand(emulator: &mut Emulator, instruction: u32) -> u32 {
    let is_immediate_value = instruction >> 25 & 1 > 0;

    if is_immediate_value {
        // Get the shift amount and the value from the instruction
        let rotate = (instruction >> 8 & 0xf) * 2;
        (instruction & 0xff).rotate_right(rotate)
    } else {
        // Determine what shifting mode will be used
        // 00: LSL Logical shift left
        // 01: LSR Logical shift right
        // 10: ASR Arithmetic shift right (sign extending)
        // 11: ROR Rotate right
        // 11, but with 0 for shift value: RRX Shift right 1 and extend.
        let shift_mode = instruction >> 5 & 3;
        // Determine if we need to fetch the shift amount from the register
        let is_register_shift = instruction >> 4 & 1 > 0;
        // Get the value from the register
        let value = emulator
            .cpu
            .get_register_value(RegisterNames::try_from(instruction & 15).unwrap());

        let shift = if is_register_shift {
            // Check to make sure that extension space instructions don't end
            // up here somehow. That is unpredictable behavior.
            let extension_space_identifier = instruction >> 7 & 1;
            assert_eq!(
                extension_space_identifier,
                0,
                "'Multiplies' extension space instructions should not enter process_shifter_operand"
            );

            // Anything above the bottom 8 bits should be ignored (because they
            // wouldn't matter anyway)
            0xff & emulator
                .cpu
                .get_register_value(RegisterNames::try_from(instruction >> 8 & 15).unwrap())
        } else {
            instruction >> 7 & 0x1f
        };

        match (shift_mode, shift) {
            (0, 0) => value,
            (0, _) => value << shift,
            (1, _) => value >> shift,
            (2, _) => value,
            (3, 0) => (if emulator.cpu.get_c() { 1 << 31 } else { 0 }) | (value >> 1),
            (3, _) => value.rotate_right(shift),
            (_, _) => panic!("Shift mode not matched for shifter_operand."),
        }
    }
}

#[derive(Copy, Clone, Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum ShiftMode {
    /// Logical shift left
    LSL = 0b00,
    /// Logical shift right
    LSR = 0b01,
    /// Arithmetic shift right (sign extending)
    ASR = 0b10,
    /// Rotate right
    ROR = 0b11,
}

pub trait BitChecker {
    fn is_bit_set(self, position: u32) -> bool;
}

impl BitChecker for u32 {
    /// Checks if the bit at the specified index is set. Will panic (in debug mode only?) if
    /// position/index is greater than 31.
    fn is_bit_set(self, position: u32) -> bool {
        ((self >> position) & 0x1) > 0
    }
}

pub trait DataProcessingInstruction {
    fn is_immediate_value(self) -> bool;
    fn get_rotate_imm(self) -> u32;
    fn get_immediate(self) -> u32;
    fn get_register_shift_imm(self) -> u32;
    fn is_register_shift(self) -> bool;
    fn get_shifter_mode(self) -> ShiftMode;
    fn get_rm(self) -> RegisterNames;
    fn get_rs(self) -> RegisterNames;
}

impl DataProcessingInstruction for u32 {
    fn is_immediate_value(self) -> bool {
        self & 0x0200_0000 > 0
    }

    fn get_rotate_imm(self) -> u32 {
        self >> 8 & 0xf
    }

    fn get_immediate(self) -> u32 {
        self & 0xff
    }

    fn get_register_shift_imm(self) -> u32 {
        self >> 7 & 0x1f
    }

    fn is_register_shift(self) -> bool {
        self & 0x10 > 0
    }

    fn get_shifter_mode(self) -> ShiftMode {
        ShiftMode::try_from(self >> 5 & 0b11).unwrap()
    }

    fn get_rm(self) -> RegisterNames {
        RegisterNames::try_from(self & 0xf).unwrap()
    }

    fn get_rs(self) -> RegisterNames {
        RegisterNames::try_from(self >> 8 & 0xf).unwrap()
    }
}

pub fn process_shifter_operand_tmp<T>(emulator: &mut Emulator, instruction: T) -> (u32, bool)
where
    T: DataProcessingInstruction + BitChecker + Copy,
{
    if instruction.is_immediate_value() {
        // Get the shift amount and the value from the instruction
        let rotate_imm = instruction.get_rotate_imm();
        let shifter_operand = (instruction.get_immediate()).rotate_right(rotate_imm * 2);
        let shifter_carry_out = if rotate_imm == 0 {
            emulator.cpu.get_c()
        } else {
            shifter_operand.is_bit_set(31)
        };
        (shifter_operand, shifter_carry_out)
    } else {
        // Determine if we need to fetch the shift amount from the register
        let is_register_shift = instruction.is_register_shift();
        // Get the value from the register
        let value = emulator.cpu.get_register_value(instruction.get_rm());

        let shift = if is_register_shift {
            // Check to make sure that extension space instructions don't end
            // up here somehow. That is unpredictable behavior.
            let extension_space_identifier = instruction.is_bit_set(7);
            assert_eq!(
                extension_space_identifier,
                false,
                "'Multiplies' extension space instructions should not enter process_shifter_operand"
            );

            // Anything above the bottom 8 bits should be ignored (because they
            // wouldn't matter anyway)
            0xff & emulator.cpu.get_register_value(instruction.get_rs())
        } else {
            instruction.get_register_shift_imm()
        };

        let shift_mode = instruction.get_shifter_mode();

        match (shift_mode, shift) {
            (ShiftMode::LSL, 0) => (value, emulator.cpu.get_c()),
            (ShiftMode::LSL, _) => match shift.cmp(&32) {
                Ordering::Less => (value << shift, value.is_bit_set(32 - shift)),
                Ordering::Equal => (0, value.is_bit_set(0)),
                Ordering::Greater => (0, false),
            },
            (ShiftMode::LSR, _) => {
                //let shifter_operand = value >> shift;
                if is_register_shift && shift == 0 {
                    (value, emulator.cpu.get_c())
                } else if shift == 0 || (is_register_shift && shift == 32) {
                    (0, value & 0x8000_0000 == 0x8000_0000)
                } else if shift < 32 {
                    (value >> shift, value.is_bit_set(shift - 1))
                } else {
                    (0, false)
                }
            }
            (ShiftMode::ASR, _) => {
                if is_register_shift && shift == 0 {
                    (value, emulator.cpu.get_c())
                } else if shift == 0 || (is_register_shift && shift >= 32) {
                    if value.is_bit_set(31) {
                        (0xFFFF_FFFF, true)
                    } else {
                        (0, false)
                    }
                } else {
                    let shifter_operand = ((value as i32) >> shift) as u32;
                    let shifter_carry_out = value.is_bit_set(shift - 1);
                    (shifter_operand, shifter_carry_out)
                }
            }
            (ShiftMode::ROR, _) => {
                if is_register_shift && shift == 0 {
                    (value, emulator.cpu.get_c())
                } else if is_register_shift && shift.trailing_zeros() >= 5 {
                    (value, value & 0x8000_0000 > 0)
                } else if shift == 0 {
                    // This is actually RRX (Rotate Right with Extend) shift mode
                    let shifter_operand =
                        (if emulator.cpu.get_c() { 1 << 31 } else { 0 }) | (value >> 1);
                    let shifter_carry_out = value.is_bit_set(0);
                    (shifter_operand, shifter_carry_out)
                } else {
                    let shifter_operand = value.rotate_right(shift & 0xf);
                    let shifter_carry_out = value.is_bit_set((shift & 0xf) - 1);
                    (shifter_operand, shifter_carry_out)
                }
            }
        }
    }
}

pub fn process_addressing_mode(emulator: &mut Emulator, instruction: u32) -> (u32, AddressingType) {
    let is_immediate_value = instruction >> 25 & 1 == 0;

    let post_index_addressing = instruction >> 24 & 1 == 0;

    // In post indexed addressing mode this would indicate that we're dealing with a T version of
    // the instruction (with Translation). These are usually used in privileged mode to emulate
    // user mode memory accesses. If that's the case then we can just ignore it.
    // In pre indexed mode this indicates whether we'll write the calculated memory address back to
    // the base register.
    let write_address_to_base_register = instruction >> 21 & 1 == 1;

    let base_register = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
    let base_register_value = emulator.cpu.get_register_value(base_register);

    let add_offset = instruction >> 23 & 1 == 1;

    let offset = if is_immediate_value {
        instruction & 0xfff
    } else {
        let shift_mode = instruction >> 5 & 0x3;
        let register_value = emulator
            .cpu
            .get_register_value(RegisterNames::try_from(instruction & 0xf).unwrap());

        let shift_imm = instruction >> 7 & 0x1f;

        match (shift_mode, shift_imm) {
            (0, 0) => register_value,
            (0, _) => register_value << shift_imm,
            (1, 0) => 0,
            (1, _) => register_value >> shift_imm,
            (2, 0) => {
                if register_value & 0x8000_0000 == 0x8000_0000 {
                    0xFFFF_FFFF
                } else {
                    0x0
                }
            }
            // Shift operations on signed integers always perform an arithmetic shift in Rust
            (2, _) => ((register_value as i32) >> shift_imm) as u32,
            (3, 0) => (if emulator.cpu.get_c() { 1 << 31 } else { 0 }) | (register_value >> 1),
            (3, _) => register_value.rotate_right(shift_imm),
            (_, _) => panic!("Shift mode not matched for shifter_operand."),
        }
    };

    let (address, _) = if add_offset {
        base_register_value.overflowing_add(offset)
    } else {
        base_register_value.overflowing_sub(offset)
    };

    if post_index_addressing {
        let temporary = emulator.cpu.get_register_value(base_register);

        // Write the calculated address back into the base register
        emulator.cpu.set_register_value(base_register, address);

        (temporary, AddressingType::PostIndexed)
    } else {
        let addressing_type = if write_address_to_base_register {
            // This should only run if the instruction condition passes, but we should never be here if
            // it doesn't so.. don't do anything special
            emulator.cpu.set_register_value(base_register, address);

            AddressingType::PreIndexed
        } else {
            AddressingType::Offset
        };

        (address, addressing_type)
    }
}

pub fn process_misc_addressing_mode(
    emulator: &mut Emulator,
    instruction: u32,
) -> (u32, AddressingType) {
    let immediate_offset = instruction >> 22 & 0x1 == 0x1;
    let add_offset = instruction >> 23 & 0x1 == 0x1;
    let post_index_addressing = instruction >> 24 & 0x1 == 0x0;

    let offset = if immediate_offset {
        let immediate_high = instruction >> 8 & 0xf;
        let immediate_low = instruction & 0xf;

        (immediate_high << 4) | immediate_low
    } else {
        emulator
            .cpu
            .get_register_value(RegisterNames::try_from(instruction & 0xf).unwrap())
    };

    let base_register = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
    let base_register_value = emulator.cpu.get_register_value(base_register);

    let (address, _) = if add_offset {
        base_register_value.overflowing_add(offset)
    } else {
        base_register_value.overflowing_sub(offset)
    };

    if post_index_addressing {
        emulator.cpu.set_register_value(base_register, address);

        (base_register_value, AddressingType::PostIndexed)
    } else {
        let write_address_to_base_register = instruction >> 21 & 0x1 == 0x1;

        let addressing_type = if write_address_to_base_register {
            emulator.cpu.set_register_value(base_register, address);
            AddressingType::PreIndexed
        } else {
            AddressingType::Offset
        };

        (address, addressing_type)
    }
}

#[inline]
pub fn addition_overflow(first: u32, second: u32, result: u32) -> bool {
    // 1. If two numbers with the sign bit OFF produce a result with the signed bit ON then overflow
    //  flag is set:
    // 0b0111_1111 + 0b0000_0001 = 0b1000_0000
    //
    //
    // 2. If two numbers with the sign bit ON produce a result with the signed bit OFF then
    //  overflow flag is set:
    // 0b1000_0000 + 0b1000_0000 = 0b0000_0000
    //
    // Truth table (32-bit):
    // | first[31] | second[31] | result[31] | overflow |
    // |-----------|------------|------------|----------|
    // | false     | false      | false      | false    |
    // | false     | false      | true       | true     | <<
    // | false     | true       | false      | false    |
    // | false     | true       | true       | false    |
    // | true      | false      | false      | false    |
    // | true      | false      | true       | false    |
    // | true      | true       | false      | true     | <<
    // | true      | true       | true       | false    |
    //
    // The first condition can be checked by XOR-ing the operands of the addition - since XOR will
    // yield false when the sign bits of the operands are the same. The second condition can also
    // be checked by XOR-ing the result with either of the operands, all we're checking is that the
    // sign bit of the result does not match the sign bits of the addition operands. If both
    // conditions are true, then the addition ended up overflowing.
    //
    // Addition with carry (e.g. why do we not need to do anything special for ADC):
    //  0b0111_1111 + 0b0 + c_flag(0b1) = 0b1000_0000
    //
    // We're only considering the first operand, the second operand and the result. If they match
    // the overflow conditions then the overflow flag is set. The carry flag does not matter at all
    // for this check.
    //
    // Let's look at some of the extremes of 8-bit addition:
    // 0b0111_1111 + 0b0111_1111 + c_flag(0b1) = 0b1111_1111
    // 0b0111_1111 + 0b0111_1111 + c_flag(0b0) = 0b1111_1110
    //
    // This tells us that when adding two positive numbers together, we'll never end up rolling
    // over into zero, no matter what the carry flag is set to.
    //
    // Similar conditions apply to negative numbers as well:
    // 0b1000_0000 + 0b1000_0000 + c_flag(0b1) = 0b1
    // 0b1000_0000 + 0b1000_0000 + c_flag(0b0) = 0b0
    //
    // These could also roll over into a negative number again due to the carry flag, but this
    // would then not result in an overflow:
    // 0b1111_1111 + 0b1000_0000 + c_flag(0b1) = 0b1000_0000
    //
    (!(first ^ second) & (second ^ result)).is_bit_set(31)
}

#[inline]
pub fn substraction_overflow(first: u32, second: u32, result: u32) -> bool {
    // Overflow check for substraction is basically the same as the overflow check for addition
    // since substraction is just addition with the second parameter negated.
    //
    // This changes the truth table (32-bit) a bit:
    // | first[31] | second[31] | result[31] | overflow |
    // |-----------|------------|------------|----------|
    // | false     | false      | false      | false    |
    // | false     | false      | true       | false    |
    // | false     | true       | false      | false    |
    // | false     | true       | true       | true     | <<
    // | true      | false      | false      | true     | <<
    // | true      | false      | true       | false    |
    // | true      | true       | false      | false    |
    // | true      | true       | true       | false    |
    //
    // Sign bits of the substraction operands now need to be different - XOR returns true. The
    // second condition remains unchanged, but only the first operand can be XOR'ed with the
    // result. Otherwise the second parameter would have to be negated first, resulting in an
    // additional operation.
    //
    ((first ^ second) & (first ^ result)).is_bit_set(31)
}

pub fn get_data_processing_operands(
    emulator: &mut Emulator,
    instruction: u32,
) -> (RegisterNames, u32, u32, bool) {
    let destination_register = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
    let operand_register_value = emulator
        .cpu
        .get_register_value(RegisterNames::try_from(instruction >> 16 & 0xf).unwrap());
    let (shifter_operand_value, shifter_carry_out) =
        process_shifter_operand_tmp(emulator, instruction);

    (
        destination_register,
        operand_register_value,
        shifter_operand_value,
        shifter_carry_out,
    )
}

/// Common addressing types
#[derive(Debug, PartialEq)]
pub enum AddressingType {
    Offset,
    PreIndexed,
    PostIndexed,
}
