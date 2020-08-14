use crate::emulator::{cpu::*, Emulator};
use instructions::*;
use std::convert::TryFrom;

/// Decodes and runs the instruction using the given emulator, and returns the
/// number of cycles used.
pub fn process_instruction(emulator: &mut Emulator, instruction: u32) -> u32 {
    // Check if the condition is met before executing the instruction.
    let condition = ConditionCodes::try_from(instruction >> 28 & 15).unwrap();
    if !emulator.cpu.check_condition(condition) {
        return 1;
    }

    decode_instruction(instruction)(emulator, instruction)
}

/// Decodes the instruction and returns the appropriate implementation.
pub fn decode_instruction(instruction: u32) -> fn(&mut Emulator, u32) -> u32 {
    // [27:20] and [7:4] are the CPU's decode bits
    // The first onces we want to look at are the three bits [27:25]
    let category = instruction >> 25 & 7;

    match category {
        // Data processing immediate shift if opcode != 0b10xx && s == 1
        // Miscellaneous instructions (Figure A3-4)
        // Data processing register shift if opcode != 0b10xx && s == 1
        // Miscellaneous instructions (Figure A3-4)
        // Multiplies (Figure A3-3) and Extra load/stores (Figure A3-5)
        0b000 | 0b001 => {
            let opcode = instruction >> 21 & 0xf;
            let set_flags = instruction >> 20 & 1;
            let lower_decode_bits = instruction >> 4 & 0xf;

            match (opcode, set_flags, lower_decode_bits) {
                (_, 1, 0b1011) if category == 0 => ldrh,
                (_, 1, 0b1101) if category == 0 => ldrsb,
                (_, 1, 0b1111) if category == 0 => ldrsh,
                (_, 0, 0b1011) if category == 0 => strh,
                (0b0000, _, 0b1001) if category == 0 => mul,
                (0b0000, _, _) => and,
                (0b0001, _, 0b1001) if category == 0 => mla,
                (0b0001, _, _) => eor,
                (0b0010, _, _) => sub,
                (0b0011, _, _) => rsb,
                (0b0100, _, 0b1001) if category == 0 => umull,
                (0b0100, _, _) => add,
                (0b0101, _, 0b1001) if category == 0 => umlal,
                (0b0101, _, _) => adc,
                (0b0110, _, 0b1001) => smull,
                (0b0110, _, _) => sbc,
                (0b0111, _, 0b1001) if category == 0 => smlal,
                (0b0111, _, _) => rsc,
                (0b1000, 0, 0b1001) if category == 0 => swp,
                (0b1010, 0, 0b1001) if category == 0 => swpb,
                (0b1000, 0, _) | (0b1010, 0, _) if category == 0 => mrs,
                (0b1000, _, _) => tst,
                (0b1001, 0, 0b0001) if category == 0 => bx,
                (0b1001, 0, _) | (0b1011, 0, _) => msr,
                (0b1001, _, _) => teq,
                (0b1010, _, _) => cmp,
                (0b1011, _, _) => cmn,
                (0b1100, _, _) => or,
                (0b1101, _, _) => mov,
                (0b1110, _, _) => bic,
                (0b1111, _, _) => mvn,
                (_, _, _) => unreachable!(),
            }
        }
        // Load/store
        // This is stupid and backward from how the dp instructions differentiate
        // between immediates and register values.
        0b010 | 0b011 => {
            let n = instruction >> 22 & 1 > 0;
            let load = instruction >> 20 & 1 > 0;
            let t = instruction >> 24 & 1 == 0 && instruction >> 21 & 1 > 0;

            match (n, load, t) {
                (false, false, true) => strt,
                (false, false, false) => str,
                (true, false, true) => strbt,
                (true, false, false) => strb,
                (false, true, true) => ldrt,
                (false, true, false) => ldr,
                (true, true, true) => ldrbt,
                (true, true, false) => ldrb,
            }
        }
        // Media instructions + architecturally undefined (Figure A3-2)
        // Architecturally undefined
        // Load/store multiple
        0b100 => {
            let load = instruction >> 20 & 1 > 0;
            match load {
                false => stm,
                true => ldm,
            }
        }
        // Branch instructions
        0b101 => {
            let link = instruction >> 24 & 1 > 0;
            match link {
                false => b,
                true => bl,
            }
        }
        // Coprocessor load/store and double register transfers
        0b110 => {
            let load = instruction >> 20 & 1 > 0;
            match load {
                false => stc,
                true => ldc,
            }
        }
        // Coprocessor data processing
        // Coprocessor register transfers
        // Software interupt
        0b111 => {
            let coprocessor_or_swi = instruction >> 24 & 1 > 0;
            let direction = instruction >> 20 & 1 > 0;
            let coprocessor_mov = instruction >> 4 & 1 > 0;
            match (coprocessor_or_swi, direction, coprocessor_mov) {
                (false, _, false) => cdp,
                (false, false, true) => mcr,
                (false, true, true) => mrc,
                (true, _, _) => swi,
            }
        }
        _ => unreachable!(),
    }
}

mod internal {
    use crate::emulator::{armv4t::utils::*, cpu::RegisterNames::*, cpu::*, Emulator};
    use std::convert::TryFrom;

    // Internal functions for reading and writing from/to memory "securely". There is nothing
    // secure about these because there is no permission checking for memory accesses on the GBA.
    // It's useful to have separate functions so that it's easier to add debug_assert's or extend
    // it them for a system with a memory protection unit.
    pub fn read_byte(emulator: &Emulator, address: u32) -> u8 {
        emulator.memory.read_byte(address)
    }

    pub fn read_half_word(emulator: &Emulator, address: u32) -> u16 {
        emulator.memory.read_half_word(address)
    }

    pub fn read_word(emulator: &Emulator, address: u32) -> u32 {
        emulator.memory.read_word(address)
    }

    pub fn write_byte(emulator: &mut Emulator, address: u32, value: u8) {
        emulator.memory.write_byte(address, value);
    }

    pub fn write_half_word(emulator: &mut Emulator, address: u32, value: u16) {
        emulator.memory.write_half_word(address, value);
    }

    pub fn write_word(emulator: &mut Emulator, address: u32, value: u32) {
        emulator.memory.write_word(address, value);
    }

    // Main bits of the load/store instructions, these are used in both the normal instructions and
    // in the with translation instructions.
    pub fn store_register(emulator: &mut Emulator, source_register: RegisterNames, address: u32) {
        let source_register_value = emulator.cpu.get_register_value(source_register);

        write_word(emulator, address & 0xFFFF_FFFC, source_register_value);
    }

    pub fn store_register_byte(
        emulator: &mut Emulator,
        source_register: RegisterNames,
        address: u32,
    ) {
        let source_register_value = emulator.cpu.get_register_value(source_register);

        write_byte(emulator, address, (source_register_value & 0xff) as u8);
    }

    pub fn load_register(
        emulator: &mut Emulator,
        destination_register: RegisterNames,
        address: u32,
    ) {
        let mut value = read_word(emulator, address & 0xFFFF_FFFC);

        // For ARMv5 and below, if the address is not word aligned, then the loaded value needs to
        // be rotated right by 8 times the value of the 2 LSB's of the address.
        let rotate = address & 0x3;
        if rotate > 0 {
            value = value.rotate_right(rotate * 8);
        }

        if destination_register == r15 {
            emulator.cpu.set_register_value(r15, value & 0xFFFF_FFFC);
        } else {
            emulator.cpu.set_register_value(destination_register, value);
        }
    }

    pub fn load_register_byte(
        emulator: &mut Emulator,
        destination_register: RegisterNames,
        address: u32,
    ) {
        let value = emulator.memory.read_byte(address);
        emulator
            .cpu
            .set_register_value(destination_register, value as u32);
    }

    pub fn load_store_instruction_wrapper(
        emulator: &mut Emulator,
        instruction: u32,
        operation: fn(&mut Emulator, RegisterNames, u32),
    ) {
        let source_or_destination_register =
            RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();

        debug_assert!(
            !(instruction >> 22 & 0x1 == 0x1
                && source_or_destination_register == r15),
            "Loads/stores: result unpredictable if source/destination register == r15 for byte operations");

        let (address, addressing_type) = process_addressing_mode(emulator, instruction);

        debug_assert!(
            !(addressing_type == AddressingType::PreIndexed
                && source_or_destination_register
                    == RegisterNames::try_from(instruction >> 16 & 0xf).unwrap()),
            "Loads/stores: result unpredictable if Rn == Rd and addressing type is PreIndexed"
        );

        operation(emulator, source_or_destination_register, address);
    }

    /// Common functionality of miscellaneous load/store instructions
    pub fn misc_load_store_instruction_wrapper(
        emulator: &mut Emulator,
        instruction: u32,
        operation: fn(&mut Emulator, RegisterNames, u32),
    ) {
        let source_or_destination_register =
            RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();

        debug_assert_ne!(
            source_or_destination_register, r15,
            "Misc loads/stores: result unpredictable if destination/source register is r15"
        );

        let (address, addressing_type) = process_misc_addressing_mode(emulator, instruction);

        debug_assert!(
            !(addressing_type == AddressingType::PreIndexed
                && source_or_destination_register
                    == RegisterNames::try_from(instruction >> 16 & 0xf).unwrap()),
            "Misc loads/stores: result unpredictable if Rn == Rd and addressing type is PreIndexed"
        );

        debug_assert!(!(instruction >> 5 & 0x1 == 0x1 && address & 0x1 == 0x1), "Misc loads/stores: unpredictable if address is not halfword aligned for halfword reads/writes");

        operation(emulator, source_or_destination_register, address);
    }

    /// Common functionality of data processing instructions
    pub fn data_processing_instruction_wrapper<T, U>(
        instruction_name: &'static str,
        emulator: &mut Emulator,
        instruction: u32,
        operation: T,
        flag_operation: U,
    ) where
        T: FnOnce(u32, u32, u32) -> u32,
        U: FnOnce(&mut Emulator, u32, u32, u32, u32),
    {
        let carry_amount = if emulator.cpu.get_c() { 1 } else { 0 };
        let should_update_flags = instruction.is_bit_set(20);

        // Get the instruction operands
        let (destination_register, operand_register_value, shifter_operand, _) =
            get_data_processing_operands(emulator, instruction);

        let result = operation(operand_register_value, shifter_operand, carry_amount);

        emulator
            .cpu
            .set_register_value(destination_register, result);

        if should_update_flags && destination_register == RegisterNames::r15 {
            if emulator.cpu.current_mode_has_spsr() {
                emulator.cpu.set_register_value(
                    RegisterNames::cpsr,
                    emulator.cpu.get_register_value(RegisterNames::spsr),
                );
            } else {
                panic!(format!("{}: unpredictable", instruction_name));
            }
        } else if should_update_flags {
            flag_operation(
                emulator,
                operand_register_value,
                shifter_operand,
                carry_amount,
                result,
            );
        }
    }

    /// Common functionality of data processing instructions used for comparing two values
    pub fn data_processing_compare_instruction_wrapper<T>(
        instruction_name: &'static str,
        emulator: &mut Emulator,
        instruction: u32,
        operation: T,
    ) where
        T: FnOnce(u32, u32) -> u32,
    {
        // Get the instruction operands
        let operand_register = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
        let operand_register_value = emulator.cpu.get_register_value(operand_register);
        let (shifter_operand, shifter_carry_out) =
            process_shifter_operand_tmp(emulator, instruction);

        let result = operation(operand_register_value, shifter_operand);

        emulator.cpu.set_nzcv(
            result.is_bit_set(31),
            result == 0,
            shifter_carry_out,    // c
            emulator.cpu.get_v(), // v: unaffected
        );
    }

    // TODO:
    // normal multiply wrapper
    // long multiply wrapper (signed and unsigned)
    // semaphore ins wrapper (swp, swpb)
    // exception generating wrapper (swi)
    // coprocessor ins wrapper
}

/// A module containing functions which implement all of the 32-bit ARM v4T
/// instructions.
pub mod instructions {
    use crate::emulator::{
        armv4t::arm::internal::*,
        armv4t::utils::*,
        cpu::{RegisterNames::*, *},
        Emulator,
    };
    use std::convert::TryFrom;

    /// Addition that includes carry from the carry bit in the CPSR register.
    pub fn adc(emulator: &mut Emulator, instruction: u32) -> u32 {
        data_processing_instruction_wrapper(
            "ADC",
            emulator,
            instruction,
            |operand_register_value, shifter_operand, carry_amount| -> u32 {
                operand_register_value
                    .wrapping_add(shifter_operand)
                    .wrapping_add(carry_amount)
            },
            |emulator, operand_register_value, shifter_operand, carry_amount, result| {
                emulator.cpu.set_nzcv(
                    result.is_bit_set(31),
                    result == 0,
                    (operand_register_value as u64)
                        .wrapping_add(shifter_operand as u64)
                        .wrapping_add(carry_amount as u64)
                        > 0xFFFF_FFFF, // c: an unsigned overflow occured
                    addition_overflow(operand_register_value, shifter_operand, result), // v: a signed overflow occured
                );
            },
        );

        // xxx: Return the actual number of cycles that the instruction should take
        5
    }

    /// Addition
    pub fn add(emulator: &mut Emulator, instruction: u32) -> u32 {
        data_processing_instruction_wrapper(
            "ADD",
            emulator,
            instruction,
            |operand_register_value, shifter_operand, _| -> u32 {
                operand_register_value.wrapping_add(shifter_operand)
            },
            |emulator, operand_register_value, shifter_operand, _, result| {
                emulator.cpu.set_nzcv(
                    result.is_bit_set(31),
                    result == 0,
                    carry_from(operand_register_value, shifter_operand), // c: carry occured
                    addition_overflow(operand_register_value, shifter_operand, result), // v: a signed overflow occured
                );
            },
        );

        // xxx: Return the actual number of cycles that the instruction should take
        5
    }

    /// Logical AND
    pub fn and(emulator: &mut Emulator, instruction: u32) -> u32 {
        let should_update_flags = instruction >> 20 & 1 > 0;

        // Get the instruction operands
        let (
            destination_register,
            operand_register_value,
            shifter_operand_value,
            shifter_carry_out,
        ) = get_data_processing_operands(emulator, instruction);

        let result = operand_register_value & shifter_operand_value;

        if should_update_flags {
            if destination_register == r15 {
                emulator
                    .cpu
                    .set_register_value(cpsr, emulator.cpu.get_register_value(spsr));
            } else {
                emulator.cpu.set_nzcv(
                    result >> 31 & 1 > 0,
                    result == 0,
                    shifter_carry_out,    // xxx: c: shifter_carry_out
                    emulator.cpu.get_v(), // xxx: this actually shouldn't be mutated at all
                );
            }
        }

        emulator
            .cpu
            .set_register_value(destination_register, result);

        // xxx: Return the actual number of cycles that the instruction should take
        5
    }

    /// Relative code branching by up 32MB in either direction.
    pub fn b(emulator: &mut Emulator, instruction: u32) -> u32 {
        let pc_value = emulator.cpu.get_register_value(r15);
        let negative = instruction >> 23 & 1 > 0;

        // The shift amount is a 26 bit two's complement integer stored in 24 bits.
        // This is all just a very complicated way to convert it to the proper 32 bit
        // two's complement integer format. We still store it as an unsigned
        // integer because otherwise Rust won't let us add them together.
        let shift = if negative {
            instruction & 0x7fffff | 0x3f80_0000
        } else {
            instruction & 0x7fffff
        } << 2;

        emulator
            .cpu
            .set_register_value(r15, pc_value.wrapping_add(shift));

        // xxx: Return the actual number of cycles that the instruction should take
        5
    }

    /// Bit clear - Equivalent to `a AND (NOT b)`
    pub fn bic(emulator: &mut Emulator, instruction: u32) -> u32 {
        let should_update_flags = instruction >> 20 & 1 > 0;

        // Get the instruction operands
        let destination_register = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
        let operand_register = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
        let (shifter_operand, shifter_carry_out) =
            process_shifter_operand_tmp(emulator, instruction);

        let result = emulator.cpu.get_register_value(operand_register) & !shifter_operand;

        if should_update_flags {
            if destination_register == r15 {
                emulator
                    .cpu
                    .set_register_value(cpsr, emulator.cpu.get_register_value(spsr));
            } else {
                emulator.cpu.set_nzcv(
                    result >> 31 & 1 > 0,
                    result == 0,
                    shifter_carry_out,
                    emulator.cpu.get_v(),
                );
            }
        }

        emulator
            .cpu
            .set_register_value(destination_register, result);

        // xxx: Return the actual number of cycles that the instruction should take
        5
    }

    /// Linked relative code branching by up 32MB in either direction. Sets r14
    /// with an address to return to after execution.
    pub fn bl(emulator: &mut Emulator, instruction: u32) -> u32 {
        let pc_value = emulator.cpu.get_register_value(r15);
        let negative = instruction >> 23 & 1 > 0;

        // The shift amount is a 26 bit two's complement integer stored in 24 bits.
        // This is all just a very complicated way to convert it to the proper 32 bit
        // two's complement integer format. We still store it as an unsigned
        // integer because otherwise Rust won't let us add them together.
        let shift = if negative {
            instruction & 0x7fffff | 0x3f80_0000
        } else {
            instruction & 0x7fffff
        } << 2;

        emulator.cpu.set_register_value(r14, pc_value);
        emulator
            .cpu
            .set_register_value(r15, pc_value.wrapping_add(shift));

        // xxx: Return the actual number of cycles that the instruction should take
        5
    }

    /// Branches execution relative to the current program counter by up 32MB in
    /// either direction. Exchanges instruction set to Thumb at the new location.
    pub fn bx(emulator: &mut Emulator, instruction: u32) -> u32 {
        let target_address_register = RegisterNames::try_from(instruction & 0xf).unwrap();

        let target_address = emulator.cpu.get_register_value(target_address_register);

        // TODO: if Rm[1:0] == 0b10 -> UNPREDICTABLE

        let thumb = target_address.is_bit_set(0);

        emulator.cpu.set_thumb_bit(thumb);

        emulator
            .cpu
            .set_register_value(r15, target_address & 0xFFFF_FFFE);

        1
    }

    /// Coprocessor data processing
    pub fn cdp(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        // Undefined instruction on the GBA
        // GBATEK: The opcodes are irrelevant for GBA/NDS7 because no coprocessor exists
        // (except for a dummy CP14 unit). However, NDS9 includes a working CP15 unit.
        // And, 3DS ARM11 uses CP10/CP11 as VFP floating point unit.

        panic!("CDP: Undefined instruction");
    }

    /// Compare negative
    pub fn cmn(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            alu_out = Rn + shifter_operand
            N Flag = alu_out[31]
            Z Flag = if alu_out == 0 then 1 else 0
            C Flag = CarryFrom(Rn + shifter_operand)
            V Flag = OverflowFrom(Rn + shifter_operand)
        */

        // Get the instruction operands
        let operand_register = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
        let operand_value = emulator.cpu.get_register_value(operand_register);
        let (shifter_operand, _) = process_shifter_operand_tmp(emulator, instruction);

        let (alu_out, overflow) = (operand_value as i32).overflowing_add(shifter_operand as i32);
        let alu_out = alu_out as u32;

        let tmp_carry = if emulator.cpu.get_c() { 1 } else { 0 };
        // Update flags if necessary
        emulator.cpu.set_nzcv(
            alu_out.is_bit_set(31),
            alu_out == 0,
            // TODO: simplify/improve
            (operand_value as u64).wrapping_add(shifter_operand as u64) > 0xFFFF_FFFF, // c: an unsigned overflow occured
            overflow, // v: signed overflow occured
        );

        1
    }

    /// Compare
    pub fn cmp(emulator: &mut Emulator, instruction: u32) -> u32 {
        // alu_out = Rn - shifter_operand
        // N Flag = alu_out[31]
        // Z Flag = if alu_out == 0 then 1 else 0
        // C Flag = NOT BorrowFrom(Rn - shifter_operand)
        // V Flag = OverflowFrom(Rn - shifter_operand)

        // Get the instruction operands
        let operand_register = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
        let operand_value = emulator.cpu.get_register_value(operand_register);
        let (shifter_operand, _) = process_shifter_operand_tmp(emulator, instruction);

        let (alu_out, overflow) = (operand_value as i32).overflowing_sub(shifter_operand as i32);
        let alu_out = alu_out as u32;

        let tmp_carry = if emulator.cpu.get_c() { 1 } else { 0 };
        // Update flags if necessary
        emulator.cpu.set_nzcv(
            alu_out.is_bit_set(31),
            alu_out == 0,
            // TODO: this was done using a wrapping_add before and it still is in other
            // instructions, however that implementation might be broken when comparing two zeroes.
            operand_value >= shifter_operand, // c: NOT BorrowFrom
            overflow,                         // v: signed overflow occured
        );

        1
    }

    /// Logical XOR
    pub fn eor(emulator: &mut Emulator, instruction: u32) -> u32 {
        // Rd = Rn EOR shifter_operand
        // if S == 1 and Rd == R15 then
        //     if CurrentModeHasSPSR() then CPSR = SPSR
        //     else UNPREDICTABLE
        // else if S == 1 then
        //     N Flag = Rd[31]
        //     Z Flag = if Rd == 0 then 1 else 0
        //     C Flag = shifter_carry_out
        //     V Flag = unaffected

        let should_update_flags = instruction >> 20 & 1 > 0;

        // Get the instruction operands
        let (
            destination_register,
            operand_register_value,
            shifter_operand_value,
            shifter_carry_out,
        ) = get_data_processing_operands(emulator, instruction);

        let result = operand_register_value ^ shifter_operand_value;

        if should_update_flags && destination_register == RegisterNames::r15 {
            if emulator.cpu.current_mode_has_spsr() {
                emulator.cpu.set_register_value(
                    RegisterNames::cpsr,
                    emulator.cpu.get_register_value(RegisterNames::spsr),
                );
            } else {
                panic!("EOR: unpredictable");
            }
        } else if should_update_flags {
            emulator.cpu.set_nzcv(
                result.is_bit_set(31),
                result == 0,
                shifter_carry_out,
                emulator.cpu.get_v(), // unaffected
            )
        }

        emulator
            .cpu
            .set_register_value(destination_register, result);

        5
    }

    /// Load coprocessor - Loads memory into a coprocessor
    pub fn ldc(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        // See CDP impl
        panic!("LDC: Undefined instruction");
    }

    /// Load Multiple
    pub fn ldm(emulator: &mut Emulator, instruction: u32) -> u32 {
        match (instruction.is_bit_set(22), instruction.is_bit_set(15)) {
            (false, _) => {
                // LDM(1)
                /*
                MemoryAccess(B-bit, E-bit)
                if ConditionPassed(cond) then
                    address = start_address

                    for i = 0 to 14
                        if register_list[i] == 1 then
                            Ri = Memory[address,4]
                            address = address + 4

                    if register_list[15] == 1 then
                        value = Memory[address,4]
                        if (architecture version 5 or above) then
                            pc = value AND 0xFFFFFFFE
                            T Bit = value[0]
                        else
                            pc = value AND 0xFFFFFFFC
                        address = address + 4
                    assert end_address == address - 4
                */
                let (start_address, end_address) =
                    process_load_store_multiple_addressing_mode(emulator, instruction);

                let register_list = instruction & 0xffff;

                let mut address = start_address;
                for pos in 0..15 {
                    if register_list.is_bit_set(pos) {
                        let register = RegisterNames::try_from(pos).unwrap();
                        let value = emulator.memory.read_word(address);
                        emulator.cpu.set_register_value(register, value);
                        address += 4;
                    }
                }

                if register_list.is_bit_set(15) {
                    let value = emulator.memory.read_word(address);
                    emulator.cpu.set_register_value(r15, value & 0xFFFF_FFFC);
                    address += 4;
                }

                // TODO: properly implement this
                assert_eq!(end_address, address - 4);
            }
            (true, false) => {
                // LDM(2)
                /*
                MemoryAccess(B-bit, E-bit)
                if ConditionPassed(cond) then
                    address = start_address
                    for i = 0 to 14
                        if register_list[i] == 1
                            Ri_usr = Memory[address,4]
                            address = address + 4
                    assert end_address == address - 4
                */
                let (start_address, end_address) =
                    process_load_store_multiple_addressing_mode(emulator, instruction);

                let register_list = instruction & 0xffff;

                let mut address = start_address;
                // TODO: fix this loop as well, 0..15 range is too error prone
                // r0 - r14
                for pos in 0..15 {
                    if register_list.is_bit_set(pos) {
                        let register = RegisterNames::try_from(pos).unwrap();
                        let value = emulator.memory.read_word(address);
                        emulator.cpu.set_register_value_in_operation_mode(
                            register,
                            value,
                            OperationModes::USR,
                        );
                        address += 4;
                    }
                }
                // TODO: properly implement this
                assert_eq!(end_address, address - 4);
            }
            (true, true) => {
                // LDM(3)
                /*
                MemoryAccess(B-bit, E-bit)
                if ConditionPassed(cond) then
                    address = start_address
                    for i = 0 to 14
                        if register_list[i] == 1 then
                            Ri = Memory[address,4]
                            address = address + 4

                    if CurrentModeHasSPSR() then
                        CPSR = SPSR
                    else
                        UNPREDICTABLE

                    value = Memory[address,4]
                    PC = value
                    address = address + 4
                    assert end_address == address - 4
                */
                let (start_address, end_address) =
                    process_load_store_multiple_addressing_mode(emulator, instruction);

                let register_list = instruction & 0xffff;

                let mut address = start_address;
                for pos in 0..15 {
                    if register_list.is_bit_set(pos) {
                        let register = RegisterNames::try_from(pos).unwrap();
                        let value = emulator.memory.read_word(address);
                        emulator.cpu.set_register_value(register, value);
                        address += 4;
                    }
                }

                if emulator.cpu.current_mode_has_spsr() {
                    let spsr_value = emulator.cpu.get_register_value(spsr);
                    emulator.cpu.set_register_value(cpsr, spsr_value);
                } else {
                    panic!("LDM(3): UNPREDICTABLE");
                }

                let value = emulator.memory.read_word(address);
                emulator.cpu.set_register_value(r15, value);
                address += 4;

                // TODO: properly implement this
                assert_eq!(end_address, address - 4);
            }
        }

        1
    }

    /// Load register
    pub fn ldr(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        MemoryAccess(B-bit, E-bit)
        if ConditionPassed(cond) then
            if (CP15_reg1_Ubit == 0) then
                data = Memory[address,4] Rotate_Right (8 * address[1:0])
            else /* CP15_reg_Ubit == 1 */
                data = Memory[address,4]
            if (Rd is R15) then
                if (ARMv5 or above) then
                    PC = data AND 0xFFFFFFFE
                    T Bit = data[0]
                else
                    PC = data AND 0xFFFFFFFC
            else
                Rd = data
        */

        load_store_instruction_wrapper(emulator, instruction, load_register);

        5
    }

    /// Load register byte
    pub fn ldrb(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        MemoryAccess(B-bit, E-bit)
        if ConditionPassed(cond) then
            Rd = Memory[address,1]
        */

        load_store_instruction_wrapper(emulator, instruction, load_register_byte);

        1
    }

    /// Load register byte with translation
    pub fn ldrbt(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            Rd = Memory[address,1]
            Rn = address
        */

        debug_assert_ne!(
            RegisterNames::try_from(instruction >> 12 & 0xf).unwrap(),
            r15,
            "LDRBT: result is unpredictable if r15 is specified for destination register"
        );

        load_store_instruction_wrapper(emulator, instruction, load_register_byte);

        1
    }

    /// Load register half-word
    pub fn ldrh(emulator: &mut Emulator, instruction: u32) -> u32 {
        misc_load_store_instruction_wrapper(
            emulator,
            instruction,
            |emulator, destination_register, address| {
                let value = read_half_word(emulator, address);
                emulator
                    .cpu
                    .set_register_value(destination_register, value as u32);
            },
        );

        1
    }

    /// Load register signed byte
    pub fn ldrsb(emulator: &mut Emulator, instruction: u32) -> u32 {
        misc_load_store_instruction_wrapper(
            emulator,
            instruction,
            |emulator, destination_register, address| {
                let value = (read_byte(emulator, address) as i32) << 24 >> 24;
                emulator
                    .cpu
                    .set_register_value(destination_register, value as u32);
            },
        );

        1
    }

    /// Load register signed halfword
    pub fn ldrsh(emulator: &mut Emulator, instruction: u32) -> u32 {
        misc_load_store_instruction_wrapper(
            emulator,
            instruction,
            |emulator, destination_register, address| {
                let value = (read_half_word(emulator, address) as i32) << 16 >> 16;
                emulator
                    .cpu
                    .set_register_value(destination_register, value as u32);
            },
        );

        1
    }

    /// Load register with translation
    pub fn ldrt(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        MemoryAccess(B-bit, E-bit)
        if ConditionPassed(cond) then
            Rd = Memory[address,4] Rotate_Right (8 * address[1:0])
        */

        debug_assert_ne!(
            RegisterNames::try_from(instruction >> 12 & 0xf).unwrap(),
            r15,
            "LDRT: result is unpredictable if r15 is specified for destination register"
        );

        load_store_instruction_wrapper(emulator, instruction, load_register);

        1
    }

    /// Move to Coprocessor from ARM Register
    pub fn mcr(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        // See CDP impl
        panic!("MCR: Undefined instruction");
    }

    /// Multiply Accumulate - multiplies two signed or unisgned 32-bit values and adds a third
    /// 32-bit value. LSB32 of the result is then are written into the destination register.
    pub fn mla(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            Rd = (Rm * Rs + Rn)[31:0]
            if S == 1 then
                N Flag = Rd[31]
                Z Flag = if Rd == 0 then 1 else 0
                C Flag = unaffected in v5 and above, UNPREDICTABLE in v4 and earlier
                V Flag = unaffected
        */

        let should_update_flags = instruction.is_bit_set(20);

        let destination_register = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
        let multiplier_register = RegisterNames::try_from(instruction & 0xf).unwrap();
        let multiplicand_register = RegisterNames::try_from(instruction >> 8 & 0xf).unwrap();
        let summand_register = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();

        // TODO: if any of the operands == r15 -> UNPREDICTABLE

        let multiplier = emulator.cpu.get_register_value(multiplier_register);
        let multiplicand = emulator.cpu.get_register_value(multiplicand_register);
        let summand = emulator.cpu.get_register_value(summand_register);

        let result = multiplier.wrapping_mul(multiplicand).wrapping_add(summand);

        emulator
            .cpu
            .set_register_value(destination_register, result);

        if should_update_flags {
            emulator.cpu.set_nzcv(
                result.is_bit_set(31),
                result == 0,
                emulator.cpu.get_c(), // c: UNPREDICTABLE
                emulator.cpu.get_v(), // v: unaffected
            )
        }

        1
    }

    /// Move
    pub fn mov(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            Rd = shifter_operand
            if S == 1 and Rd == R15 then
                if CurrentModeHasSPSR() then
                    CPSR = SPSR
                else UNPREDICTABLE
            else if S == 1 then
                N Flag = Rd[31]
                Z Flag = if Rd == 0 then 1 else 0
                C Flag = shifter_carry_out
                V Flag = unaffected
        */

        let should_update_flags = instruction >> 20 & 1 > 0;

        let destination_register = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
        let (shifter_operand, shifter_carry_out) =
            process_shifter_operand_tmp(emulator, instruction);

        emulator
            .cpu
            .set_register_value(destination_register, shifter_operand);

        if should_update_flags && destination_register == RegisterNames::r15 {
            if emulator.cpu.current_mode_has_spsr() {
                emulator.cpu.set_register_value(
                    RegisterNames::cpsr,
                    emulator.cpu.get_register_value(RegisterNames::spsr),
                );
            } else {
                panic!("MOV: unpredictable");
            }
        } else if should_update_flags {
            emulator.cpu.set_nzcv(
                // TODO: should get values from Rd here
                shifter_operand.is_bit_set(31),
                shifter_operand == 0,
                shifter_carry_out,    // c: carry out
                emulator.cpu.get_v(), // v: unaffected
            );
        }

        1
    }

    /// Move to ARM Register from Coprocessor
    pub fn mrc(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        // See CDP impl
        panic!("MRC: Undefined instruction");
    }

    /// Move PSR to general-purpose register
    pub fn mrs(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            if R == 1 then
                Rd = SPSR
            else
                Rd = CPSR
        */

        let destination_register = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
        let move_spsr = instruction.is_bit_set(22);

        let value = if move_spsr {
            // Accessing SPSR in User mode or System mode is UNPREDICTABLE
            emulator.cpu.get_register_value(spsr)
        } else {
            emulator.cpu.get_register_value(cpsr)
        };

        emulator.cpu.set_register_value(destination_register, value);

        1
    }

    /// Move to Status Register From ARM Register
    pub fn msr(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            if opcode[25] == 1 then
                operand = 8_bit_immediate Rotate_Right (rotate_imm * 2)
            else
                operand = Rm
            if (operand AND UnallocMask) !=0 then
                UNPREDICTABLE              /* Attempt to set reserved bits */
            byte_mask = (if field_mask[0] == 1 then 0x000000FF else 0x00000000) OR
                        (if field_mask[1] == 1 then 0x0000FF00 else 0x00000000) OR
                        (if field_mask[2] == 1 then 0x00FF0000 else 0x00000000) OR
                        (if field_mask[3] == 1 then 0xFF000000 else 0x00000000)
            if R == 0 then
                if InAPrivilegedMode() then
                    if (operand AND StateMask) != 0 then
                        UNPREDICTABLE      /* Attempt to set non-ARM execution state */
                    else
                        mask = byte_mask AND (UserMask OR PrivMask)
                else
                    mask = byte_mask AND UserMask
                CPSR = (CPSR AND NOT mask) OR (operand AND mask)
            else  /* R == 1 */
                if CurrentModeHasSPSR() then
                    mask = byte_mask AND (UserMask OR PrivMask OR StateMask)
                    SPSR = (SPSR AND NOT mask) OR (operand AND mask)
                else
                    UNPREDICTABLE
        */

        let immediate_operand = instruction.is_bit_set(25);

        let operand = if immediate_operand {
            let immediate_8bit_value = instruction & 0xff;
            let rotate_imm = instruction >> 8 & 0xf;
            immediate_8bit_value.rotate_right(rotate_imm << 1)
        } else {
            let source_register = RegisterNames::try_from(instruction & 0xf).unwrap();
            emulator.cpu.get_register_value(source_register)
        };

        #[repr(u32)]
        enum BitMaskConstants4T {
            Unalloc = 0x0FFF_FF00,
            User = 0xF000_0000,
            Priv = 0x0000_000F,
            State = 0x0000_0020,
        }

        // if operand & BitMaskConstants4T::Unalloc as u32 != 0 {
        //     // TODO: panic probably not the appropriate thing to do here.. especially since the
        //     // field_mask bits allow for the reserved bytes to be modified.
        //     panic!("MSR: Unpredictable (1)");
        // }

        let field_mask = (instruction >> 16) & 0xf;

        // TODO: must be a simpler way to do this
        let mut byte_mask: u32 = 0;
        for pos in 0..4 {
            if field_mask.is_bit_set(pos) {
                // pos << 3 to multiply pos by 8
                byte_mask |= 0xFF << (pos << 3);
            }
        }

        let write_spsr = instruction.is_bit_set(22);

        if !write_spsr {
            // If we're in a privileged operation mode (anything that's not User Mode)
            let mask = if emulator.cpu.get_operation_mode().expect("TODO") != OperationModes::USR {
                if operand & BitMaskConstants4T::State as u32 != 0 {
                    panic!("MSR: Unpredictable (2)");
                } else {
                    byte_mask & (BitMaskConstants4T::User as u32 | BitMaskConstants4T::Priv as u32)
                }
            } else {
                byte_mask & BitMaskConstants4T::User as u32
            };
            let cpsr_value = emulator.cpu.get_register_value(cpsr);
            let result = (cpsr_value & (!mask)) | (operand & mask);
            emulator.cpu.set_register_value(cpsr, result);
        } else if emulator.cpu.current_mode_has_spsr() {
            let mask = byte_mask
                & (BitMaskConstants4T::User as u32
                    | BitMaskConstants4T::Priv as u32
                    | BitMaskConstants4T::State as u32);
            let spsr_value = emulator.cpu.get_register_value(spsr);
            let result = (spsr_value & (!mask)) | (operand & mask);
            emulator.cpu.set_register_value(spsr, result);
        } else {
            panic!("MSR: Unpredictable (3)");
        }

        1
    }

    /// Multiply
    pub fn mul(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            if S == 1 then
                N Flag = Rd[31]
                Z Flag = if Rd == 0 then 1 else 0
                C Flag = unaffected in v5 and above, UNPREDICTABLE in v4 and earlier
                V Flag = unaffected
        */

        let carry_amount = if !emulator.cpu.get_c() { 1 } else { 0 };
        let should_update_flags = instruction >> 20 & 1 > 0;

        // Get the instruction operands
        let destination_register = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
        let first_operand_register = RegisterNames::try_from(instruction & 0xf).unwrap();
        let second_operand_register = RegisterNames::try_from(instruction >> 8 & 0xf).unwrap();

        let first_operand_value = emulator.cpu.get_register_value(first_operand_register);
        let second_operand_value = emulator.cpu.get_register_value(second_operand_register);

        let result = first_operand_value.wrapping_mul(second_operand_value);

        // TODO: if Rd, Rm or Rs == r15 -> UNPREDICTABLE

        emulator
            .cpu
            .set_register_value(destination_register, result);

        if should_update_flags {
            emulator.cpu.set_nzcv(
                result.is_bit_set(31),
                result == 0,
                emulator.cpu.get_c(), // c: UNPREDICTABLE
                emulator.cpu.get_v(), // v: unaffected
            );
        }

        1
    }

    /// Move Not (generates a logical ones complement of a value)
    pub fn mvn(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            Rd = NOT shifter_operand
            if S == 1 and Rd == R15 then
                if CurrentModeHasSPSR() then
                    CPSR = SPSR
                else UNPREDICTABLE
            else if S == 1 then
                N Flag = Rd[31]
                Z Flag = if Rd == 0 then 1 else 0
                C Flag = shifter_carry_out
                V Flag = unaffected
        */

        let should_update_flags = instruction >> 20 & 1 > 0;

        let destination_register = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
        let (shifter_operand, shifter_carry_out) =
            process_shifter_operand_tmp(emulator, instruction);

        let shifter_operand = !shifter_operand;

        emulator
            .cpu
            .set_register_value(destination_register, shifter_operand);

        if should_update_flags && destination_register == RegisterNames::r15 {
            if emulator.cpu.current_mode_has_spsr() {
                emulator.cpu.set_register_value(
                    RegisterNames::cpsr,
                    emulator.cpu.get_register_value(RegisterNames::spsr),
                );
            } else {
                panic!("MVN: unpredictable");
            }
        } else if should_update_flags {
            emulator.cpu.set_nzcv(
                // TODO: should get values from Rd here
                shifter_operand.is_bit_set(31),
                shifter_operand == 0,
                shifter_carry_out,    // c: carry out
                emulator.cpu.get_v(), // v: unaffected
            );
        }

        1
    }

    /// Logical OR (also referred to as the orr instruction)
    pub fn or(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            Rd = Rn OR shifter_operand
            if S == 1 and Rd == R15 then
                if CurrentModeHasSPSR() then
                    CPSR = SPSR
                else UNPREDICTABLE
            else if S == 1 then
                N Flag = Rd[31]
                Z Flag = if Rd == 0 then 1 else 0
                C Flag = shifter_carry_out
                V Flag = unaffected
        */

        let should_update_flags = instruction >> 20 & 1 > 0;

        let destination_register = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
        let operand_register = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
        let operand_value = emulator.cpu.get_register_value(operand_register);

        let (shifter_operand, shifter_carry_out) =
            process_shifter_operand_tmp(emulator, instruction);

        let result = operand_value | shifter_operand;

        emulator
            .cpu
            .set_register_value(destination_register, result);

        if should_update_flags && destination_register == RegisterNames::r15 {
            if emulator.cpu.current_mode_has_spsr() {
                emulator.cpu.set_register_value(
                    RegisterNames::cpsr,
                    emulator.cpu.get_register_value(RegisterNames::spsr),
                );
            } else {
                panic!("ORR: unpredictable");
            }
        } else if should_update_flags {
            emulator.cpu.set_nzcv(
                // TODO: should get values from Rd here
                result.is_bit_set(31),
                result == 0,
                shifter_carry_out,    // c: carry out
                emulator.cpu.get_v(), // v: unaffected
            );
        }

        1
    }

    /// Reverse substract
    pub fn rsb(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            Rd = shifter_operand - Rn
            if S == 1 and Rd == R15 then
                if CurrentModeHasSPSR() then
                    CPSR = SPSR
                else UNPREDICTABLE
            else if S == 1 then
                N Flag = Rd[31]
                Z Flag = if Rd == 0 then 1 else 0
                C Flag = NOT BorrowFrom(shifter_operand - Rn)
                V Flag = OverflowFrom(shifter_operand - Rn)
        */

        let should_update_flags = instruction >> 20 & 1 > 0;

        // Get the instruction operands
        let destination_register = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
        let operand_register = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
        let operand_register_value = emulator.cpu.get_register_value(operand_register);
        let (shifter_operand, _) = process_shifter_operand_tmp(emulator, instruction);

        let result = shifter_operand.wrapping_sub(operand_register_value);

        emulator
            .cpu
            .set_register_value(destination_register, result);

        if should_update_flags && destination_register == RegisterNames::r15 {
            if emulator.cpu.current_mode_has_spsr() {
                emulator.cpu.set_register_value(
                    RegisterNames::cpsr,
                    emulator.cpu.get_register_value(RegisterNames::spsr),
                );
            } else {
                panic!("RSB: unpredictable");
            }
        } else if should_update_flags {
            emulator.cpu.set_nzcv(
                result.is_bit_set(31),
                result == 0,
                // TODO: is this correct? in some places this is implemented incorrectly
                shifter_operand >= operand_register_value, // c: NOT BorrowFrom
                substraction_overflow(shifter_operand, operand_register_value, result), // v: signed overflow occured
            );
        }

        1
    }

    /// Reverse Substract with Carry
    pub fn rsc(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            Rd = shifter_operand - Rn - NOT(C Flag)
            if S == 1 and Rd == R15 then
                if CurrentModeHasSPSR() then
                    CPSR = SPSR
                else UNPREDICTABLE
            else if S == 1 then
                N Flag = Rd[31]
                Z Flag = if Rd == 0 then 1 else 0
                C Flag = NOT BorrowFrom(shifter_operand - Rn - NOT(C Flag))
                V Flag = OverflowFrom(shifter_operand - Rn - NOT(C Flag))
        */

        let carry_amount = if !emulator.cpu.get_c() { 1 } else { 0 };
        let should_update_flags = instruction >> 20 & 1 > 0;

        // Get the instruction operands
        let destination_register = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
        let operand_register = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
        let operand_register_value = emulator.cpu.get_register_value(operand_register);
        let (shifter_operand, _) = process_shifter_operand_tmp(emulator, instruction);

        let result = shifter_operand
            .wrapping_sub(operand_register_value)
            .wrapping_sub(carry_amount);

        if should_update_flags && destination_register == RegisterNames::r15 {
            if emulator.cpu.current_mode_has_spsr() {
                emulator.cpu.set_register_value(
                    RegisterNames::cpsr,
                    emulator.cpu.get_register_value(RegisterNames::spsr),
                );
            } else {
                panic!("RSC: unpredictable");
            }
        } else if should_update_flags {
            emulator.cpu.set_nzcv(
                result.is_bit_set(31),
                result == 0,
                // TODO: verify that this is ok
                (shifter_operand as u64) >= (operand_register_value as u64 + carry_amount as u64), // c: NOT BorrowFrom
                substraction_overflow(shifter_operand, operand_register_value, result), // v: signed overflow occured
            );
        }

        emulator
            .cpu
            .set_register_value(destination_register, result);

        1
    }

    /// Substract with Carry
    pub fn sbc(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            Rd = Rn - shifter_operand - NOT(C Flag)
            if S == 1 and Rd == R15 then
                if CurrentModeHasSPSR() then
                    CPSR = SPSR
                else UNPREDICTABLE
            else if S == 1 then
                N Flag = Rd[31]
                Z Flag = if Rd == 0 then 1 else 0
                C Flag = NOT BorrowFrom(Rn - shifter_operand - NOT(C Flag))
                V Flag = OverflowFrom(Rn - shifter_operand - NOT(C Flag))
        */

        let carry_amount = if !emulator.cpu.get_c() { 1 } else { 0 };
        let should_update_flags = instruction >> 20 & 1 > 0;

        // Get the instruction operands
        let destination_register = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
        let operand_register = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
        let (shifter_operand, _) = process_shifter_operand_tmp(emulator, instruction);

        let result = emulator
            .cpu
            .get_register_value(operand_register)
            .wrapping_sub(shifter_operand)
            .wrapping_sub(carry_amount);

        if should_update_flags && destination_register == RegisterNames::r15 {
            if emulator.cpu.current_mode_has_spsr() {
                emulator.cpu.set_register_value(
                    RegisterNames::cpsr,
                    emulator.cpu.get_register_value(RegisterNames::spsr),
                );
            } else {
                panic!("SBC: unpredictable");
            }
        } else if should_update_flags {
            let tmp_carry = if emulator.cpu.get_c() { 1 } else { 0 };
            // Update flags if necessary
            emulator.cpu.set_nzcv(
                result.is_bit_set(31),
                result == 0,
                // TODO: clean this up
                (emulator.cpu.get_register_value(operand_register) as u64)
                    .wrapping_add((!shifter_operand) as u64 + tmp_carry as u64)
                    > 0xFFFF_FFFF, // c: NOT BorrowFrom
                substraction_overflow(
                    emulator.cpu.get_register_value(operand_register),
                    shifter_operand,
                    result,
                ), // v: signed overflow occured
            );
        }

        emulator
            .cpu
            .set_register_value(destination_register, result);

        1
    }

    /// Signed Multiply Accumulate Long
    pub fn smlal(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            RdLo = (Rm * Rs)[31:0] + RdLo /* Signed multiplication */
            RdHi = (Rm * Rs)[63:32] + RdHi + CarryFrom((Rm * Rs)[31:0] + RdLo)
            if S == 1 then
                N Flag = RdHi[31]
                Z Flag = if (RdHi == 0) and (RdLo == 0) then 1 else 0
                C Flag = unaffected    /* See "C and V flags" note */
                V Flag = unaffected    /* See "C and V flags" note */
        */

        let should_update_flags = instruction.is_bit_set(20);

        let destination_register_high = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
        let destination_register_low = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();

        let multiplier_register = RegisterNames::try_from(instruction & 0xf).unwrap();
        let multiplicand_register = RegisterNames::try_from(instruction >> 8 & 0xf).unwrap();

        // TODO: if any of the operands == r15 -> UNPREDICTABLE
        // TODO: RdHi == RdLo -> UNPREDICTABLE

        // Cast the multiplier and multiplicand to signed integers because we need to perform
        // signed multiplication. This is important in this case because we don't ignore the upper
        // 32 bits of the 64 bit result.
        let multiplier = emulator.cpu.get_register_value(multiplier_register) as i32;
        let multiplicand = emulator.cpu.get_register_value(multiplicand_register) as i32;

        let rd_lo_value = emulator.cpu.get_register_value(destination_register_low);
        let rd_hi_value = emulator.cpu.get_register_value(destination_register_high);

        let result = (multiplier as i64).wrapping_mul(multiplicand as i64);
        let result_low = (result as u32).wrapping_add(rd_lo_value);
        let carry_amount = if carry_from(result as u32, rd_lo_value) {
            1
        } else {
            0
        };

        let result_high = ((result >> 32) as u32)
            .wrapping_add(rd_hi_value)
            .wrapping_add(carry_amount);

        emulator
            .cpu
            .set_register_value(destination_register_low, result_low);
        emulator
            .cpu
            .set_register_value(destination_register_high, result_high);

        if should_update_flags {
            emulator.cpu.set_nzcv(
                result_high.is_bit_set(31),
                result_low == 0 && result_high == 0,
                emulator.cpu.get_c(), // c: UNPREDICTABLE
                emulator.cpu.get_v(), // v: unaffected
            )
        }

        1
    }

    /// Signed Multiply Long
    pub fn smull(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            RdHi = (Rm * Rs)[63:32] /* Signed multiplication */
            RdLo = (Rm * Rs)[31:0]
            if S == 1 then
                N Flag = RdHi[31]
                Z Flag = if (RdHi == 0) and (RdLo == 0) then 1 else 0
                C Flag = unaffected    /* See "C and V flags" note */
                V Flag = unaffected    /* See "C and V flags" note */
        */

        let should_update_flags = instruction.is_bit_set(20);

        let destination_register_high = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
        let destination_register_low = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();

        let multiplier_register = RegisterNames::try_from(instruction & 0xf).unwrap();
        let multiplicand_register = RegisterNames::try_from(instruction >> 8 & 0xf).unwrap();

        // TODO: if any of the operands == r15 -> UNPREDICTABLE
        // TODO: RdHi == RdLo -> UNPREDICTABLE

        // Cast the multiplier and multiplicand to signed integers because we need to perform
        // signed multiplication. This is important in this case because we don't ignore the upper
        // 32 bits of the 64 bit result.
        let multiplier = emulator.cpu.get_register_value(multiplier_register) as i32;
        let multiplicand = emulator.cpu.get_register_value(multiplicand_register) as i32;

        let result = (multiplier as i64).wrapping_mul(multiplicand as i64);
        let result_low = result as u32;
        let result_high = (result >> 32) as u32;

        emulator
            .cpu
            .set_register_value(destination_register_low, result_low);
        emulator
            .cpu
            .set_register_value(destination_register_high, result_high);

        if should_update_flags {
            emulator.cpu.set_nzcv(
                result_high.is_bit_set(31),
                result_low == 0 && result_high == 0,
                emulator.cpu.get_c(), // c: UNPREDICTABLE
                emulator.cpu.get_v(), // v: unaffected
            )
        }

        1
    }

    /// Store Coprocessor
    pub fn stc(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        // See CDP impl
        panic!("STC: Undefined instruction");
    }

    /// Store Multiple
    pub fn stm(emulator: &mut Emulator, instruction: u32) -> u32 {
        // TODO: If Rn == R15 -> UNPREDICTABLE

        // TODO: simplify this, the two impls are copy-pastes with a single line difference
        match instruction.is_bit_set(22) {
            false => {
                // STM(1)
                /*
                MemoryAccess(B-bit, E-bit)
                processor_id = ExecutingProcessor()
                if ConditionPassed(cond) then
                    address = start_address
                    for i = 0 to 15
                        if register_list[i] == 1 then
                            Memory[address,4] = Ri
                            address = address + 4
                            if Shared(address) then   /* from ARMv6 */
                                physical_address = TLB(address)
                                ClearExclusiveByAddress(physical_address,processor_id,4)
                                /* See Summary of operation on page A2-49 */
                    assert end_address == address - 4
                */
                let (start_address, end_address) =
                    process_load_store_multiple_addressing_mode(emulator, instruction);

                let register_list = instruction & 0xffff;

                let mut address = start_address;

                for pos in 0..16 {
                    if register_list.is_bit_set(pos) {
                        let register = RegisterNames::try_from(pos).unwrap();
                        let value = emulator.cpu.get_register_value(register);

                        // WARN: See `Reading the program counter A2-9`
                        // TODO: the above also needs to be considered in STR implementation
                        // When STR or STM store R15 the the address that it contains is
                        // incremented by either 0x8 or 0xC. GBA seems to always increment it by 12
                        // (search for `PC+12` in GBATEK docs), at least for load/store instructions.
                        let value = if register == r15 { value + 12 } else { value };

                        emulator.memory.write_word(address, value);
                        address += 4;
                    }
                }

                assert_eq!(end_address, address - 4);
            }
            true => {
                // STM(2)
                /*
                MemoryAccess(B-bit, E-bit)
                processor_id = ExecutingProcessor()
                if ConditionPassed(cond) then
                    address = start_address
                    for i = 0 to 15
                        if register_list[i] == 1
                            Memory[address,4] = Ri_usr
                            address = address + 4
                            if Shared(address) then    /* from ARMv6 */
                                physical_address = TLB(address)
                                ClearExclusiveByAddress(physical_address,processor_id,4)
                                /* See Summary of operation on page A2-49 */
                    assert end_address == address - 4
                */
                // Base address is always read from the current processor mode register, not the
                // User mode registers.
                let (start_address, end_address) =
                    process_load_store_multiple_addressing_mode(emulator, instruction);

                let register_list = instruction & 0xffff;

                let mut address = start_address;

                for pos in 0..16 {
                    if register_list.is_bit_set(pos) {
                        let register = RegisterNames::try_from(pos).unwrap();
                        let value = emulator
                            .cpu
                            .get_register_value_in_operation_mode(register, OperationModes::USR);

                        let value = if register == r15 { value + 12 } else { value };

                        emulator.memory.write_word(address, value);
                        address += 4;
                    }
                }

                assert_eq!(end_address, address - 4);
            }
        }

        1
    }

    /// Store register
    pub fn str(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        MemoryAccess(B-bit, E-bit)
        processor_id = ExecutingProcessor()
        if ConditionPassed(cond) then
            Memory[address,4] = Rd
            if Shared(address) then /* from ARMv6 */
                physical_address = TLB(address)
                ClearExclusiveByAddress(physical_address,processor_id,4)
                /* See Summary of operation on page A2-49 */
        */

        // Probably don't need to worry about the processor ID etc. because there is only one
        // processor which interacts with the MMU (hopefully?). The ClearExclusiveByAddress is used
        // to clear any requests for exclusive access, from an address range, for all processors,
        // other than the one specified by processor ID. This is only valid for ARM architectures
        // from ARMv6 onwards.

        // MemoryAccess(B-bit, E-bit) defines the endian model (see Glossary-9).

        load_store_instruction_wrapper(emulator, instruction, store_register);

        1
    }

    /// Store register byte
    pub fn strb(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        processor_id = ExecutingProcessor()
        if ConditionPassed(cond) then
            Memory[address,1] = Rd[7:0]
        */

        load_store_instruction_wrapper(emulator, instruction, store_register_byte);

        1
    }

    /// Store register byte with translation
    pub fn strbt(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        processor_id = ExecutingProcessor()
        if ConditionPassed(cond) then
            Memory[address,1] = Rd[7:0]
        */

        load_store_instruction_wrapper(emulator, instruction, store_register_byte);

        1
    }

    /// Store half-word
    pub fn strh(emulator: &mut Emulator, instruction: u32) -> u32 {
        misc_load_store_instruction_wrapper(
            emulator,
            instruction,
            |emulator, source_register, address| {
                let value = emulator.cpu.get_register_value(source_register) & 0xffff;

                write_half_word(emulator, address, value as u16);
            },
        );

        1
    }

    /// Store register with translation
    pub fn strt(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        MemoryAccess(B-bit, E-bit)
        processor_id = ExecutingProcessor()
        if ConditionPassed(cond) then
            Memory[address,4] = Rd
        */

        load_store_instruction_wrapper(emulator, instruction, store_register);

        1
    }

    /// Substraction
    pub fn sub(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            Rd = Rn - shifter_operand
            if S == 1 and Rd == R15 then
                if CurrentModeHasSPSR() then
                    CPSR = SPSR
                else UNPREDICTABLE
            else if S == 1 then
                N Flag = Rd[31]
                Z Flag = if Rd == 0 then 1 else 0
                C Flag = NOT BorrowFrom(Rn - shifter_operand)
                V Flag = OverflowFrom(Rn - shifter_operand)
        */

        let should_update_flags = instruction >> 20 & 1 > 0;

        let (destination_register, operand_register_value, shifter_operand_value, _) =
            get_data_processing_operands(emulator, instruction);

        // The overflow flag is only relevant when dealing with signed numbers. ALU of course
        // doesn't care but Rust's unsigned `overflowing_sub` does not always return the overflow
        // flag when you would expect it to be set.
        let (value, overflow) =
            (operand_register_value as i32).overflowing_sub(shifter_operand_value as i32);

        emulator
            .cpu
            .set_register_value(destination_register, value as u32);

        if should_update_flags && destination_register == RegisterNames::r15 {
            if emulator.cpu.current_mode_has_spsr() {
                emulator.cpu.set_register_value(
                    RegisterNames::cpsr,
                    emulator.cpu.get_register_value(RegisterNames::spsr),
                );
            } else {
                // Supposedly unpredictable behaviour but the CPU might be able to deal with it, in
                // a perfectly predictable way... Worry about it later, if it actually ever happens.
                panic!("SUB: unpredictable");
            }
        } else if should_update_flags {
            emulator.cpu.set_nzcv(
                value >> 31 & 1 > 0,
                value == 0,
                operand_register_value >= shifter_operand_value,
                overflow,
            );
        }

        1
    }

    /// Triggers an interupt vector from software. Usually used to make system
    /// calls into the BIOS.
    pub fn swi(emulator: &mut Emulator, _instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            R14_svc   = address of next instruction after the SWI instruction
            SPSR_svc  = CPSR
            CPSR[4:0] = 0b10011              /* Enter Supervisor mode */
            CPSR[5]   = 0                    /* Execute in ARM state */
            /* CPSR[6] is unchanged */
            CPSR[7]   = 1                    /* Disable normal interrupts */
            /* CPSR[8] is unchanged */
            CPSR[9] = CP15_reg1_EEbit
            if high vectors configured then
                PC    = 0xFFFF0008
            else
                PC    = 0x00000008
        */

        // TODO: might not be fully correct

        // TODO: this is not safe
        let old_cpsr = emulator.cpu.get_register_value(cpsr);

        // Enter supervisor mode
        emulator.cpu.set_operation_mode(OperationModes::SVC);
        let next_instruction_address = emulator.cpu.get_register_value(r15) + 4;

        // Store next instruction address and CPSR
        emulator
            .cpu
            .set_register_value(r14, next_instruction_address);
        emulator.cpu.set_register_value(spsr, old_cpsr);

        emulator.cpu.set_fiq_disable(true);
        emulator.cpu.set_irq_disable(false);

        emulator.cpu.set_register_value(r15, 0x0000_0008);

        1
    }

    /// Swap
    pub fn swp(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        MemoryAccess(B-bit, E-bit)
        processor_id = ExecutingProcessor()
        if ConditionPassed(cond) then
            if (CP15_reg1_Ubit == 0) then
                temp = Memory[address,4] Rotate_Right (8 * address[1:0])
                Memory[address,4] = Rm
                Rd = temp
            else /* CP15_reg1_Ubit ==1 */
                temp = Memory[address,4]
                Memory[address,4] = Rm
                Rd = temp
            if Shared(address) then   /* ARMv6 */
                physical_address = TLB(address)
                ClearExclusiveByAddress(physical_address,processor_id,4)
                /* See Summary of operation on page A2-49 */
        */

        // CP15_reg1_Ubit is referring to the bit for enabling unaligned data access operations.
        // This is set to 0 for legacy code (assuming this is the case on the GBA). Unaligned loads
        // are treated as rotated aligned data accesses.

        let destination_register = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
        let load_address_register = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
        let value_to_store_register = RegisterNames::try_from(instruction & 0xf).unwrap();

        let value_to_store = emulator.cpu.get_register_value(value_to_store_register);
        let load_address = emulator.cpu.get_register_value(load_address_register);

        let temp = read_word(emulator, load_address & 0xFFFF_FFFC);

        let rotate = load_address & 0x3;
        let temp = if rotate > 0 {
            temp.rotate_right(rotate * 8)
        } else {
            temp
        };

        write_word(emulator, load_address & 0xFFFF_FFFC, value_to_store);

        emulator.cpu.set_register_value(destination_register, temp);

        1
    }

    /// Swap Byte
    pub fn swpb(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        MemoryAccess(B-bit, E-bit)
        processor_id = ExecutingProcessor()
        if ConditionPassed(cond) then
            temp = Memory[address,1]
            Memory[address,1] = Rm[7:0]
            Rd = temp
            if Shared(address) then    /* ARMv6 */
                physical_address = TLB(address)
                ClearExclusiveByAddress(physical_address,processor_id,1)
                /* See Summary of operation on page A2-49 */
        */

        let destination_register = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
        let load_address_register = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
        let value_to_store_register = RegisterNames::try_from(instruction & 0xf).unwrap();

        // We're only interested in the 8 least significant bits
        let value_to_store = emulator.cpu.get_register_value(value_to_store_register) as u8;
        let load_address = emulator.cpu.get_register_value(load_address_register);

        let temp = read_byte(emulator, load_address);

        write_byte(emulator, load_address, value_to_store);

        // Zero extend the value read from memory to an unsigned 32-bit integer and store it in the
        // destination register.
        emulator
            .cpu
            .set_register_value(destination_register, temp as u32);

        1
    }

    /// Test equivalence
    pub fn teq(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            alu_out = Rn EOR shifter_operand
            N Flag = alu_out[31]
            Z Flag = if alu_out == 0 then 1 else 0
            C Flag = shifter_carry_out
            V Flag = unaffected
        */

        data_processing_compare_instruction_wrapper(
            "TEQ",
            emulator,
            instruction,
            |operand_register_value, shifter_operand| -> u32 {
                operand_register_value ^ shifter_operand
            },
        );

        1
    }

    /// Test
    pub fn tst(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            alu_out = Rn AND shifter_operand
            N Flag = alu_out[31]
            Z Flag = if alu_out == 0 then 1 else 0
            C Flag = shifter_carry_out
            V Flag = unaffected
        */

        data_processing_compare_instruction_wrapper(
            "TST",
            emulator,
            instruction,
            |operand_register_value, shifter_operand| -> u32 {
                operand_register_value & shifter_operand
            },
        );

        1
    }

    /// Unsigned Multiply Accumulate Long
    pub fn umlal(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            RdLo = (Rm * Rs)[31:0] + RdLo    /* Unsigned multiplication */
            RdHi = (Rm * Rs)[63:32] + RdHi + CarryFrom((Rm * Rs)[31:0] + RdLo)
            if S == 1 then
                N Flag = RdHi[31]
                Z Flag = if (RdHi == 0) and (RdLo == 0) then 1 else 0
                C Flag = unaffected    /* See "C and V flags" note */
                V Flag = unaffected    /* See "C and V flags" note */
        */
        let should_update_flags = instruction.is_bit_set(20);

        let destination_register_high = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
        let destination_register_low = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();

        let multiplier_register = RegisterNames::try_from(instruction & 0xf).unwrap();
        let multiplicand_register = RegisterNames::try_from(instruction >> 8 & 0xf).unwrap();

        // TODO: if any of the operands == r15 -> UNPREDICTABLE
        // TODO: RdHi == RdLo -> UNPREDICTABLE

        let multiplier = emulator.cpu.get_register_value(multiplier_register);
        let multiplicand = emulator.cpu.get_register_value(multiplicand_register);

        let rd_lo_value = emulator.cpu.get_register_value(destination_register_low);
        let rd_hi_value = emulator.cpu.get_register_value(destination_register_high);

        // Unsigned multiplication
        let result = (multiplier as u64).wrapping_mul(multiplicand as u64);
        let result_low = (result as u32).wrapping_add(rd_lo_value);
        let carry_amount = if carry_from(result as u32, rd_lo_value) {
            1
        } else {
            0
        };

        let result_high = ((result >> 32) as u32)
            .wrapping_add(rd_hi_value)
            .wrapping_add(carry_amount);

        emulator
            .cpu
            .set_register_value(destination_register_low, result_low);
        emulator
            .cpu
            .set_register_value(destination_register_high, result_high);

        if should_update_flags {
            emulator.cpu.set_nzcv(
                result_high.is_bit_set(31),
                result_low == 0 && result_high == 0,
                emulator.cpu.get_c(), // c: UNPREDICTABLE
                emulator.cpu.get_v(), // v: unaffected
            )
        }

        1
    }

    /// Unsigned Multiply Long
    pub fn umull(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        if ConditionPassed(cond) then
            RdHi = (Rm * Rs)[63:32]    /* Unsigned multiplication */
            RdLo = (Rm * Rs)[31:0]
            if S == 1 then
                N Flag = RdHi[31]
                Z Flag = if (RdHi == 0) and (RdLo == 0) then 1 else 0
                C Flag = unaffected    /* See "C and V flags" note */
                V Flag = unaffected    /* See "C and V flags" note */
        */

        let should_update_flags = instruction.is_bit_set(20);

        let destination_register_high = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
        let destination_register_low = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();

        let multiplier_register = RegisterNames::try_from(instruction & 0xf).unwrap();
        let multiplicand_register = RegisterNames::try_from(instruction >> 8 & 0xf).unwrap();

        // TODO: if any of the operands == r15 -> UNPREDICTABLE
        // TODO: RdHi == RdLo -> UNPREDICTABLE

        let multiplier = emulator.cpu.get_register_value(multiplier_register);
        let multiplicand = emulator.cpu.get_register_value(multiplicand_register);

        // Unsigned multiplication
        let result = (multiplier as u64).wrapping_mul(multiplicand as u64);
        let result_low = result as u32;
        let result_high = (result >> 32) as u32;

        emulator
            .cpu
            .set_register_value(destination_register_low, result_low);
        emulator
            .cpu
            .set_register_value(destination_register_high, result_high);

        if should_update_flags {
            emulator.cpu.set_nzcv(
                result_high.is_bit_set(31),
                result_low == 0 && result_high == 0,
                emulator.cpu.get_c(), // c: UNPREDICTABLE
                emulator.cpu.get_v(), // v: unaffected
            )
        }

        1
    }
}
