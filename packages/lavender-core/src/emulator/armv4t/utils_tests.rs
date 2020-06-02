use crate::emulator::{armv4t::utils::*, cpu::RegisterNames::*, Emulator};

#[test]
fn test_addressing_mode_2_immediate_offset() {
    let mut emulator = Emulator::dummy();

    emulator.cpu.set_register_value(r1, 0x4000_0000);

    // Add offset to base register value
    {
        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0101_1001_0001_0011_1000_0000_0001 - ldr r3,[r1,0x801]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE591_3801);
        assert_eq!(address, 0x4000_0801);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
    }

    // Substract offset from base register value
    {
        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0101_0001_0001_0011_1000_0000_0001 - ldr r3,[r1,-0x801]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE511_3801);
        assert_eq!(address, 0x3FFF_F7FF);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
    }
}

#[test]
fn test_addressing_mode_2_immediate_postindexed() {
    let mut emulator = Emulator::dummy();

    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0100_1001_0001_0011_1000_0000_0001 - ldr r3,[r1],0x801
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE491_3801);
        assert_eq!(address, 0x4000_0000);
        assert_eq!(addressing_type, AddressingType::PostIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0801);
    }

    // Substract offset
    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0100_0001_0001_0011_1000_0000_0001 - ldr r3,[r1],-0x801
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE411_3801);
        assert_eq!(address, 0x4000_0000);
        assert_eq!(addressing_type, AddressingType::PostIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x3FFF_F7FF);
    }
}

#[test]
fn test_addressing_mode_2_immediate_preindexed() {
    let mut emulator = Emulator::dummy();

    // Add offset
    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);
        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0101_1011_0001_0011_1000_0000_0001 - ldr r3,[r1,0x801]!
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE5B1_3801);
        assert_eq!(address, 0x4000_0801);
        assert_eq!(addressing_type, AddressingType::PreIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0801);
    }

    // Substract offset
    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);
        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0101_0011_0001_0011_1000_0000_0001 - ldr r3,[r1,-0x801]!
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE531_3801);
        assert_eq!(address, 0x3FFF_F7FF);
        assert_eq!(addressing_type, AddressingType::PreIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x3FFF_F7FF);
    }
}

#[test]
fn test_addressing_mode_2_register_offset() {
    let mut emulator = Emulator::dummy();

    emulator.cpu.set_register_value(r1, 0x4000_0000);
    emulator.cpu.set_register_value(r2, 0x1000_0001);

    // Add offset
    {
        //   cond    P UBWL Rn   Rd   SBZ       Rm
        // 0b1110_0111_1001_0001_0011_0000_0000_0001 - ldr r3,[r1,r2]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_3002);
        assert_eq!(address, 0x5000_0001);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1000_0001);
    }

    // Substract offset
    {
        //   cond    P UBWL Rn   Rd   SBZ       Rm
        // 0b1110_0111_0001_0001_0011_0000_0000_0001 - ldr r3,[r1,-r2]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE711_3002);
        assert_eq!(address, 0x2FFF_FFFF);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1000_0001);
    }
}

#[test]
fn test_addressing_mode_2_register_preindexed() {
    let mut emulator = Emulator::dummy();

    emulator.cpu.set_register_value(r2, 0x1000_0001);

    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);
        //   cond    P UBWL Rn   Rd   SBZ       Rm
        // 0b1110_0111_1011_0001_0011_0000_0000_0010 - ldr r3,[r1,r2]!
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE7B1_3002);
        assert_eq!(address, 0x5000_0001);
        assert_eq!(addressing_type, AddressingType::PreIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x5000_0001);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1000_0001);
    }

    // Substract offset
    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);
        //   cond    P UBWL Rn   Rd   SBZ       Rm
        // 0b1110_0111_0011_0001_0011_0000_0000_0010 - ldr r3,[r1,-r2]!
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE731_3002);
        assert_eq!(address, 0x2FFF_FFFF);
        assert_eq!(addressing_type, AddressingType::PreIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x2FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1000_0001);
    }
}

#[test]
fn test_addressing_mode_2_scaled_register_offset_asr() {
    let mut emulator = Emulator::dummy();

    emulator.cpu.set_register_value(r1, 0x4000_0000);
    emulator.cpu.set_register_value(r2, 0x8000_0000);

    {
        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1001_0001_0011_11110_10_0_0010 - ldr r3,[r1,r2,asr 0x1E]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_3F42);
        assert_eq!(address, 0x3FFF_FFFE);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
    }

    // Special case when shift_imm == 0 and Rm contains a negative number
    {
        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1001_0001_0011_00000_10_0_0010 - ldr r3,[r1,r2,asr 0x20]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_3042);
        assert_eq!(address, 0x3FFF_FFFF);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
    }

    // Special case when shift_imm == 0 and Rm contains a positive number
    {
        emulator.cpu.set_register_value(r2, 0x7FFF_FFFF);

        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1001_0001_0011_00000_10_0_0010 - ldr r3,[r1,r2,asr 0x20]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_3042);
        assert_eq!(address, 0x4000_0000);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x7FFF_FFFF);
    }
}

#[test]
fn test_addressing_mode_2_scaled_register_offset_lsl() {
    let mut emulator = Emulator::dummy();

    emulator.cpu.set_register_value(r1, 0x4000_0000);
    emulator.cpu.set_register_value(r2, 0x0000_0001);

    // Add offset
    {
        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1001_0001_0011_11100_00_0_0010 - ldr r3,[r1,r2,lsl 0x1C]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_3E02);
        assert_eq!(address, 0x5000_0000);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0000_0001);
    }

    // Substract offset
    {
        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_0001_0001_0011_11100_00_0_0010 - ldr r3,[r1,-r2,lsl 0x1C]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE711_3E02);
        assert_eq!(address, 0x3000_0000);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0000_0001);
    }
}

#[test]
fn test_addressing_mode_2_scaled_register_offset_lsr() {
    let mut emulator = Emulator::dummy();

    emulator.cpu.set_register_value(r1, 0x4000_0000);
    emulator.cpu.set_register_value(r2, 0x8000_0000);

    // Add offset
    {
        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1001_0001_0011_00011_01_0_0010 - ldr r3,[r1,r2,lsr 0x3]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_31A2);
        assert_eq!(address, 0x5000_0000);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
    }

    // Special case when shift_imm == 0 (i.e. "32 bit shift")
    {
        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1001_0001_0011_00000_01_0_0010 - ldr r3,[r1,r2,lsr 0x20]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_3022);
        assert_eq!(address, 0x4000_0000);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
    }
}

#[test]
fn test_addressing_mode_2_scaled_register_offset_ror_rrx() {
    let mut emulator = Emulator::dummy();

    emulator.cpu.set_register_value(r1, 0x4000_0000);
    emulator.cpu.set_register_value(r2, 0x0001_1000);

    {
        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1001_0001_0011_10000_11_0_0010 - ldr r3,[r1,r2,ror 0x10]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_3862);
        assert_eq!(address, 0x5000_0001);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0001_1000);
    }

    // Special case when shift_imm == 0 and C flag is set
    {
        emulator.cpu.set_nzcv(false, false, true, false);
        emulator.cpu.set_register_value(r2, 0x2);

        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1001_0001_0011_00000_11_0_0010 - ldr r3,[r1,r2,rrx 0x1]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_3062);
        assert_eq!(address, 0xC000_0001);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x2);
    }

    // Special case when shift_imm == 0 and C flag is not set
    {
        emulator.cpu.set_nzcv(false, false, false, false);
        emulator.cpu.set_register_value(r2, 0x2);

        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1001_0001_0011_00000_11_0_0010 - ldr r3,[r1,r2,rrx 0x1]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_3062);
        assert_eq!(address, 0x4000_0001);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x2);
    }
}

#[test]
fn test_addressing_mode_2_scaled_register_preindexed_lsl() {
    let mut emulator = Emulator::dummy();

    emulator.cpu.set_register_value(r1, 0x4000_0000);
    emulator.cpu.set_register_value(r2, 0x0000_0001);

    {
        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1011_0001_0011_11100_00_0_0010 - ldr r3,[r1,r2,lsl 0x1C]!
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE7B1_3E02);
        assert_eq!(address, 0x5000_0000);
        assert_eq!(addressing_type, AddressingType::PreIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x5000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0000_0001);
    }
}

#[test]
fn test_addressing_mode_3_immediate_offset() {
    let mut emulator = Emulator::dummy();

    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);

        //   cond    P U WL Rn   Rd   immH  SH  immL
        // 0b1110_0001_1101_0001_0010_1000_1011_0001 - ldrh r2,[r1,0x81]
        let (address, addressing_type) = process_misc_addressing_mode(&mut emulator, 0xE1D1_28B1);
        assert_eq!(address, 0x4000_0081);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
    }

    // Substract offset
    {
        //   cond    P U WL Rn   Rd   immH  SH  immL
        // 0b1110_0001_0101_0001_0010_1000_1011_0001 - ldrh r2,[r1,-0x81]
        let (address, addressing_type) = process_misc_addressing_mode(&mut emulator, 0xE151_28B1);
        assert_eq!(address, 0x3FFF_FF7F);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
    }
}

#[test]
fn test_addressing_mode_3_register_offset() {
    let mut emulator = Emulator::dummy();

    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);
        emulator.cpu.set_register_value(r3, 0x1000_0000);

        //   cond    P U WL Rn   Rd   SBZ   SH  Rm
        // 0b1110_0001_1001_0001_0010_0000_1011_0011 - ldrh r2,[r1,r3]
        let (address, addressing_type) = process_misc_addressing_mode(&mut emulator, 0xE191_20B3);
        assert_eq!(address, 0x5000_0000);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
    }
}

#[test]
fn test_addressing_mode_3_immediate_preindexed() {
    let mut emulator = Emulator::dummy();

    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);

        //   cond    P U WL Rn   Rd   immH  SH  Rm
        // 0b1110_0001_1111_0001_0010_1000_1011_0001 - ldrh r2,[r1,0x81]!
        let (address, addressing_type) = process_misc_addressing_mode(&mut emulator, 0xE1F1_28B1);
        assert_eq!(address, 0x4000_0081);
        assert_eq!(addressing_type, AddressingType::PreIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0081);
    }
}

#[test]
fn test_addressing_mode_3_register_preindexed() {
    let mut emulator = Emulator::dummy();

    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);
        emulator.cpu.set_register_value(r3, 0x1000_0000);

        //   cond    P U WL Rn   Rd   SBZ   SH  Rm
        // 0b1110_0001_1011_0001_0010_0000_1011_0011 - ldrh r2,[r1,r3]!
        let (address, addressing_type) = process_misc_addressing_mode(&mut emulator, 0xE1B1_20B3);
        assert_eq!(address, 0x5000_0000);
        assert_eq!(addressing_type, AddressingType::PreIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x5000_0000);
    }
}

#[test]
fn test_addressing_mode_3_immediate_postindexed() {
    let mut emulator = Emulator::dummy();

    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);

        //   cond    P U WL Rn   Rd   immH  SH  Rm
        // 0b1110_0000_1101_0001_0010_1000_1011_0001 - ldrh r2,[r1],0x81
        let (address, addressing_type) = process_misc_addressing_mode(&mut emulator, 0xE0D1_28B1);
        assert_eq!(address, 0x4000_0000);
        assert_eq!(addressing_type, AddressingType::PostIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0081);
    }
}

#[test]
fn test_addressing_mode_3_register_postindexed() {
    let mut emulator = Emulator::dummy();

    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);
        emulator.cpu.set_register_value(r3, 0x1000_0000);

        //   cond    P U WL Rn   Rd   SBZ   SH  Rm
        // 0b1110_0000_1001_0001_0010_0000_1011_0011 - ldrh r2,[r1],r3
        let (address, addressing_type) = process_misc_addressing_mode(&mut emulator, 0xE091_20B3);
        assert_eq!(address, 0x4000_0000);
        assert_eq!(addressing_type, AddressingType::PostIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x5000_0000);
    }
}

#[test]
fn test_addressing_mode_1_immediate() {
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_nzcv(false, false, true, false);

        //   cond    opc  S Rn   Rd   rot  imm8
        // 0x1110_0011_1011_0000_0000_0000_1000_0000 - movs r0,0x80
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE3B0_0080);
        assert_eq!(shifter_operand, 0x80);
        assert_eq!(shifter_carry_out, true);
    }

    {
        let mut emulator = Emulator::dummy();

        //   cond    opc  S Rn   Rd   rot  imm8
        // 0b1110_0011_1011_0000_0000_0001_0000_0010 - movs r0,0x80000000
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE3B0_0102);
        assert_eq!(shifter_operand, 0x8000_0000);
        assert_eq!(shifter_carry_out, true);
    }

    {
        let mut emulator = Emulator::dummy();

        //   cond    opc  S Rn   Rd   rot  imm8
        // 0b1110_0011_1011_0000_0000_0100_0000_0011 - movs r0,3000000h
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE3B0_0403);
        assert_eq!(shifter_operand, 0x0300_0000);
        assert_eq!(shifter_carry_out, false);
    }
}

#[test]
fn test_addressing_mode_1_register() {
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xAABB_CCDD);
        emulator.cpu.set_nzcv(false, false, true, false);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_0000_0001 - movs r0,r1
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0001);
        assert_eq!(shifter_operand, 0xAABB_CCDD);
        assert_eq!(shifter_carry_out, true);
    }

    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xCCDD_EEFF);
        emulator.cpu.set_nzcv(false, false, false, false);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_0000_0001 - movs r0,r1
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0001);
        assert_eq!(shifter_operand, 0xCCDD_EEFF);
        assert_eq!(shifter_carry_out, false);
    }
}

#[test]
fn test_addressing_mode_1_lsl_immediate() {
    // movs r0,r1,lsl 0h
    // is actually the same as above ^
    //
    // movs r0,r1,lsl 1h
    // r1 = 0x8000_0000
    // shifter_carry_out = true
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x8000_0000);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_1000_0001 - movs r0,r1,lsl 1h
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0081);
        assert_eq!(shifter_operand, 0x0);
        assert_eq!(shifter_carry_out, true);
    }
    //
    // movs r0,r1,lsl 2h
    // r1 = 0xBFFF_FFFF
    // shifter_carry_out = false
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xBFFF_FFFF);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0001_0000_0001 - movs r0,r1,lsl 2h
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0101);
        assert_eq!(shifter_operand, 0xFFFF_FFFC);
        assert_eq!(shifter_carry_out, false);
    }
}

#[test]
fn test_addressing_mode_1_lsl_register() {
    // movs r0,r1,lsl r2
    // r2 = 0
    // shifter_operand = r1
    // shifter_carry_out = c_flag
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xAABB_CCDD);
        emulator.cpu.set_register_value(r2, 0);
        emulator.cpu.set_nzcv(false, false, true, false);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0010_0001_0010 - movs r0,r1,lsl r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0211);
        assert_eq!(shifter_operand, 0xAABB_CCDD);
        assert_eq!(shifter_carry_out, true);
    }
    //
    // r2 = 0x1
    // r1 = 0x8000_0000
    // shifter_operand = 0
    // shifter_carry_out = true
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x8000_0000);
        emulator.cpu.set_register_value(r2, 0x1);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0010_0001_0010 - movs r0,r1,lsl r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0211);
        assert_eq!(shifter_operand, 0x0);
        assert_eq!(shifter_carry_out, true);
    }
    //
    // r2 = 0x2
    // r1 = 0xBFFF_FFFF
    // shifter_operand = 0xFFFF_FFFC
    // shifter_carry_out = false
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xBFFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x2);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0010_0001_0010 - movs r0,r1,lsl r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0211);
        assert_eq!(shifter_operand, 0xFFFF_FFFC);
        assert_eq!(shifter_carry_out, false);
    }
    //
    // r2 = 0x20
    // r1 = 0x1
    // shifter_operand = 0
    // shifter_carry_out = true
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x1);
        emulator.cpu.set_register_value(r2, 0x20);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0010_0001_0010 - movs r0,r1,lsl r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0211);
        assert_eq!(shifter_operand, 0x0);
        assert_eq!(shifter_carry_out, true);
    }
    //
    // r2 = 0x20
    // r1 = 0xFFFF_FFFE
    // shifter_operand = 0
    // shifter_carry_out = false
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFE);
        emulator.cpu.set_register_value(r2, 0x20);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0010_0001_0010 - movs r0,r1,lsl r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0211);
        assert_eq!(shifter_operand, 0x0);
        assert_eq!(shifter_carry_out, false);
    }
    //
    // r2 = 0x21
    // r1 = 0xFFFF_FFFF
    // shifter_operand = 0
    // shifter_carry_out = false
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x21);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0010_0001_0010 - movs r0,r1,lsl r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0211);
        assert_eq!(shifter_operand, 0x0);
        assert_eq!(shifter_carry_out, false);
    }
}

#[test]
fn test_addressing_mode_1_lsr_immediate() {
    // movs r0,r1,lsr 20h
    // 0xE1B0_0021
    //
    // r1 = 0x8000_0000
    // shifter_operand = 0
    // shifter_carry_out = true
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x8000_0000);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_0010_0001 - movs r0,r1,lsr 0x20
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0021);
        assert_eq!(shifter_operand, 0x0);
        assert_eq!(shifter_carry_out, true);
    }
    //
    // r1 = 0x7FFF_FFFF
    // shifter_operand = 0
    // shifter_carry_out = false
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_0010_0001 - movs r0,r1,lsr 0x20
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0021);
        assert_eq!(shifter_operand, 0x0);
        assert_eq!(shifter_carry_out, false);
    }

    // movs r0,r1,lsr 1h
    // 0xE1B0_00A1
    //
    // r1 = 0x3
    // shifter_operand = 0x1
    // shifter_carry_out = true
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x3);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_1010_0001 - movs r0,r1,lsr 1h
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_00A1);
        assert_eq!(shifter_operand, 0x1);
        assert_eq!(shifter_carry_out, true);
    }
    //
    // r1 = 0xFFFF_FFFE
    // shifter_operand = 0x7FFF_FFFF
    // shifter_carry_out = false
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFE);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_1010_0001 - movs r0,r1,lsr 1h
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_00A1);
        assert_eq!(shifter_operand, 0x7FFF_FFFF);
        assert_eq!(shifter_carry_out, false);
    }
}

#[test]
fn test_addressing_mode_1_lsr_register() {
    // movs r0,r1,lsr r2
    // 0xE1B0_0231
    //
    // r2 = 0xFFFF_FF00
    // r1 = 0xAABB_CCDD
    // shifter_operand = r1
    // shifter_carry_out = c_flag
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xAABB_CCDD);
        emulator.cpu.set_register_value(r2, 0xFFFF_FF00);
        emulator.cpu.set_nzcv(false, false, true, false);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_1010_0001 - movs r0,r1,lsr r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0231);
        assert_eq!(shifter_operand, 0xAABB_CCDD);
        assert_eq!(shifter_carry_out, true);
    }
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xAABB_CCDD);
        emulator.cpu.set_register_value(r2, 0xFFFF_FF00);
        emulator.cpu.set_nzcv(false, false, false, false);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_1010_0001 - movs r0,r1,lsr r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0231);
        assert_eq!(shifter_operand, 0xAABB_CCDD);
        assert_eq!(shifter_carry_out, false);
    }
    //
    // r2 = 0x2
    // r1 = 0x6
    // shifter_operand = 0x1
    // shifter_carry_out = true
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x6);
        emulator.cpu.set_register_value(r2, 0x2);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_1010_0001 - movs r0,r1,lsr r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0231);
        assert_eq!(shifter_operand, 0x1);
        assert_eq!(shifter_carry_out, true);
    }
    //
    // r2 = 0x20
    // r1 = 0x8000_0000
    // shifter_operand = 0x0
    // shifter_carry_out = true
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x8000_0000);
        emulator.cpu.set_register_value(r2, 0x20);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_1010_0001 - movs r0,r1,lsr r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0231);
        assert_eq!(shifter_operand, 0x0);
        assert_eq!(shifter_carry_out, true);
    }
    //
    // r2 = 0x20
    // r1 = 0x7FFF_FFFF
    // shifter_operand = 0x0
    // shifter_carry_out = false
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x20);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_1010_0001 - movs r0,r1,lsr r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0231);
        assert_eq!(shifter_operand, 0x0);
        assert_eq!(shifter_carry_out, false);
    }
    //
    // r2 = 0x21
    // r1 = 0xFFFF_FFFF
    // shifter_operand = 0x0
    // shifter_carry_out = false
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x21);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_1010_0001 - movs r0,r1,lsr r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0231);
        assert_eq!(shifter_operand, 0x0);
        assert_eq!(shifter_carry_out, false);
    }
}

#[test]
fn test_addressing_mode_1_asr_immediate() {
    // movs r0,r1,asr 20h
    // 0xE1B0_0041
    //
    // r1 = 0x8000_0000
    // shifter_operand = 0xFFFF_FFFF
    // shifter_carry_out = true
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x8000_0000);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_0100_0001 - movs r0,r1,asr 0x20
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0041);
        assert_eq!(shifter_operand, 0xFFFF_FFFF);
        assert_eq!(shifter_carry_out, true);
    }
    //
    // r1 = 0x7FFF_FFFF
    // shifter_operand = 0x0
    // shifter_carry_out = false
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_0100_0001 - movs r0,r1,asr 0x20
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0041);
        assert_eq!(shifter_operand, 0x0);
        assert_eq!(shifter_carry_out, false);
    }

    // movs r0,r1,asr 2h
    // 0xE1B0_0141
    //
    // r1 = 0x6
    // shifter_operand = 0x1
    // shifter_carry_out = true
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x6);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0001_0100_0001 - movs r0,r1,asr 0x2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0141);
        assert_eq!(shifter_operand, 0x1);
        assert_eq!(shifter_carry_out, true);
    }

    // movs r0,r1,asr 1h
    // 0xE1B0_00C1
    //
    // r1 = 0xFFFF_FFFE
    // shifter_operand = 0xFFFF_FFFF
    // shifter_carry_out = false
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFE);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0001_0100_0001 - movs r0,r1,asr 0x1
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_00C1);
        assert_eq!(shifter_operand, 0xFFFF_FFFF);
        assert_eq!(shifter_carry_out, false);
    }
}

#[test]
fn test_addressing_mode_1_asr_register() {
    // movs r0,r1,asr r2
    // 0xE1B0_0251
    //
    // r2 = 0
    // shifter_operand = r1
    // shifter_carry_out = c_flag
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xAABB_CCDD);
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_nzcv(false, false, true, false);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0001_0100_0001 - movs r0,r1,asr r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0251);
        assert_eq!(shifter_operand, 0xAABB_CCDD);
        assert_eq!(shifter_carry_out, true);
    }
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xAABB_CCDD);
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_nzcv(false, false, false, false);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0001_0100_0001 - movs r0,r1,asr r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0251);
        assert_eq!(shifter_operand, 0xAABB_CCDD);
        assert_eq!(shifter_carry_out, false);
    }
    //
    // r2 = 1
    // r1 = 0x1
    // shifter_operand = 0x0
    // shifter_carry_out = true
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x1);
        emulator.cpu.set_register_value(r2, 0x1);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0001_0100_0001 - movs r0,r1,asr r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0251);
        assert_eq!(shifter_operand, 0x0);
        assert_eq!(shifter_carry_out, true);
    }
    //
    // r2 = 1
    // r1 = 0xFFFF_FFFE
    // shifter_operand = 0xFFFF_FFFF
    // shifter_carry_out = false
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFE);
        emulator.cpu.set_register_value(r2, 0x1);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0001_0100_0001 - movs r0,r1,asr r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0251);
        assert_eq!(shifter_operand, 0xFFFF_FFFF);
        assert_eq!(shifter_carry_out, false);
    }
    //
    // r2 = 32
    // r1 = 0x7FFF_FFFF
    // shifter_operand = 0
    // shifter_carry_out = false
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x20);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0001_0100_0001 - movs r0,r1,asr r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0251);
        assert_eq!(shifter_operand, 0x0);
        assert_eq!(shifter_carry_out, false);
    }
    //
    // r2 = 32
    // r1 = 0x8000_0000
    // shifter_operand = 0xFFFF_FFFF
    // shifter_carry_out = true
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x8000_0000);
        emulator.cpu.set_register_value(r2, 0x20);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0001_0100_0001 - movs r0,r1,asr r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0251);
        assert_eq!(shifter_operand, 0xFFFF_FFFF);
        assert_eq!(shifter_carry_out, true);
    }
}

#[test]
fn test_addressing_mode_1_ror_immediate() {
    // movs r0,r1,ror 1h
    // 0xE1B0_00E1
    //
    // r1 = 0x1
    // shifter_operand = 0x8000_0000
    // shifter_carry_out = true
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x1);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_1110_0001 - movs r0,r1,ror 0x1
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_00E1);
        assert_eq!(shifter_operand, 0x8000_0000);
        assert_eq!(shifter_carry_out, true);
    }
    //
    // r1 = 0xFFFF_FFFE
    // shifter_operand = 0x7FFF_FFFF
    // shifter_carry_out = false
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFE);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_1110_0001 - movs r0,r1,ror 0x1
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_00E1);
        assert_eq!(shifter_operand, 0x7FFF_FFFF);
        assert_eq!(shifter_carry_out, false);
    }
}

#[test]
fn test_addressing_mode_1_ror_register() {
    // movs r0,r1,ror r2
    // 0xE1B0_0271
    //
    // r2 = 0xFFFF_FF00
    // r1 = 0xAABB_CCDD
    // shifter_operand = r1
    // shifter_carry_out = c_flag
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xAABB_CCDD);
        emulator.cpu.set_register_value(r2, 0xFFFF_FF00);
        emulator.cpu.set_nzcv(false, false, true, false);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0010_0111_0001 - movs r0,r1,ror r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0271);
        assert_eq!(shifter_operand, 0xAABB_CCDD);
        assert_eq!(shifter_carry_out, true);
    }
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xAABB_CCDD);
        emulator.cpu.set_register_value(r2, 0xFFFF_FF00);
        emulator.cpu.set_nzcv(false, false, false, false);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0010_0111_0001 - movs r0,r1,ror r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0271);
        assert_eq!(shifter_operand, 0xAABB_CCDD);
        assert_eq!(shifter_carry_out, false);
    }
    //
    // r2 = 0xFFFF_FFE0
    // r1 = 0x8000_0000
    // shifter_operand = 0x8000_0000
    // shifter_carry_out = true
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x8000_0000);
        emulator.cpu.set_register_value(r2, 0xFFFF_FFE0);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0010_0111_0001 - movs r0,r1,ror r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0271);
        assert_eq!(shifter_operand, 0x8000_0000);
        assert_eq!(shifter_carry_out, true);
    }
    //
    // r2 = 0xFFFF_FFE0
    // r1 = 0x7FFF_FFFF
    // shifter_operand = 0x7FFF_FFFF
    // shifter_carry_out = false
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0xFFFF_FFE0);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0010_0111_0001 - movs r0,r1,ror r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0271);
        assert_eq!(shifter_operand, 0x7FFF_FFFF);
        assert_eq!(shifter_carry_out, false);
    }
    //
    // r2 = 0x1
    // r1 = 0x3
    // shifter_operand = 0x8000_0001
    // shifter_carry_out = true
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x3);
        emulator.cpu.set_register_value(r2, 0x1);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0010_0111_0001 - movs r0,r1,ror r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0271);
        assert_eq!(shifter_operand, 0x8000_0001);
        assert_eq!(shifter_carry_out, true);
    }
    //
    // r2 = 0x1
    // r1 = 0xFFFF_FFFE
    // shifter_operand = 0x7FFF_FFFF
    // shifter_carry_out = false
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFE);
        emulator.cpu.set_register_value(r2, 0x1);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0010_0111_0001 - movs r0,r1,ror r2
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0271);
        assert_eq!(shifter_operand, 0x7FFF_FFFF);
        assert_eq!(shifter_carry_out, false);
    }
}

#[test]
fn test_addressing_mode_1_rrx() {
    // This is actually just movs r0,r1,rrx - the immediate parameter is always 0
    // movs r0,r1,rrx 0x1
    // 0xE1B0_0061
    //
    // r1 = 0x3
    // c_flag = true
    // shifter_operand = 0x8000_0001
    // shifter_carry_out = true
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x3);
        emulator.cpu.set_nzcv(false, false, true, false);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_0110_0001 - movs r0,r1,rrx 0x1
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0061);
        assert_eq!(shifter_operand, 0x8000_0001);
        assert_eq!(shifter_carry_out, true);
    }
    //
    // r1 = 0x2
    // c_flag = true
    // shifter_operand = 0x8000_0001
    // shifter_carry_out = false
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x2);
        emulator.cpu.set_nzcv(false, false, true, false);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_0110_0001 - movs r0,r1,rrx 0x1
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0061);
        assert_eq!(shifter_operand, 0x8000_0001);
        assert_eq!(shifter_carry_out, false);
    }
    //
    // r1 = 0x3
    // c_flag = false
    // shifter_operand = 0x8000_0001
    // shifter_carry_out = true
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x3);
        emulator.cpu.set_nzcv(false, false, false, false);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_0110_0001 - movs r0,r1,rrx 0x1
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0061);
        assert_eq!(shifter_operand, 0x1);
        assert_eq!(shifter_carry_out, true);
    }
    //
    // r1 = 0x2
    // c_flag = false
    // shifter_operand = 0x1
    // shifter_carry_out = false
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0x2);
        emulator.cpu.set_nzcv(false, false, false, false);

        //   cond    opc  S Rn   Rd             Rm
        // 0b1110_0001_1011_0000_0000_0000_0110_0001 - movs r0,r1,rrx 0x1
        let (shifter_operand, shifter_carry_out) = process_shifter_operand_tmp(&mut emulator, 0xE1B0_0061);
        assert_eq!(shifter_operand, 0x1);
        assert_eq!(shifter_carry_out, false);
    }
}

