use crate::emulator::{
    armv4t::arm::{decode_instruction, instructions::*, process_instruction},
    cpu::OperationModes,
    cpu::RegisterNames::*,
    Emulator,
};

#[test]
fn decode_adc() {
    assert_eq!(decode_instruction(0x0_0a_000_0_0) as usize, adc as usize);
}

#[test]
fn behavior_adc() {
    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF + 0x1 + c_flag == 0x8000_0001
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x1);
        emulator.cpu.set_nzcv(false, false, true, false);

        //   cond    P UBWL Rn   Rd   offset12
        // 0x1110_0000_1011_0001_0000_0000_0000_0010 - adcs r0,r1,r2
        process_instruction(&mut emulator, 0xE0B1_0002);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x8000_0001);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xFFFF_FFFF + 0x2 == 0x1
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x2);
        emulator.cpu.set_nzcv(false, false, false, false);

        //   cond    P UBWL Rn   Rd   offset12
        // 0x1110_0000_1011_0001_0000_0000_0000_0010 - adcs r0,r1,r2
        process_instruction(&mut emulator, 0xE0B1_0002);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x2);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xFFFF_FFFF + 0x1 == 0x0
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x1);
        emulator.cpu.set_nzcv(false, false, false, false);

        //   cond    P UBWL Rn   Rd   offset12
        // 0x1110_0000_1011_0001_0000_0000_0000_0010 - adcs r0,r1,r2
        process_instruction(&mut emulator, 0xE0B1_0002);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 + 0x7FFF_FFFF + c_flag == 0x0
        emulator.cpu.set_register_value(r1, 0x8000_0000);
        emulator.cpu.set_register_value(r2, 0x7FFF_FFFF);
        emulator.cpu.set_nzcv(false, false, true, false);

        //   cond    P UBWL Rn   Rd   offset12
        // 0x1110_0000_1011_0001_0000_0000_0000_0010 - adcs r0,r1,r2
        process_instruction(&mut emulator, 0xE0B1_0002);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF + 0x8000_0000 + c_flag == 0x0
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x8000_0000);
        emulator.cpu.set_nzcv(false, false, true, false);

        //   cond    P UBWL Rn   Rd   offset12
        // 0x1110_0000_1011_0001_0000_0000_0000_0010 - adcs r0,r1,r2
        process_instruction(&mut emulator, 0xE0B1_0002);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF + 0x0 + c_flag == 0x8000_0000
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_nzcv(false, false, true, false);

        //   cond    P UBWL Rn   Rd   offset12
        // 0x1110_0000_1011_0001_0000_0000_0000_0010 - adcs r0,r1,r2
        process_instruction(&mut emulator, 0xE0B1_0002);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), true);
    }
}

#[test]
fn decode_add() {
    assert_eq!(decode_instruction(0x0_08_000_0_0) as usize, add as usize);
}

#[test]
fn behavior_add() {
    let mut emulator = Emulator::dummy();

    // Initialize r3 adn r4 as the accumulators, and r5 as the increment for r4
    emulator.cpu.set_register_value(r3, 0);
    emulator.cpu.set_register_value(r4, 0);
    emulator.cpu.set_register_value(r5, 3);

    for _ in 0..10 {
        // add r3, r3, #1
        process_instruction(&mut emulator, 0b1110_00_1_0100_1_0011_0011_0000_00000001);
        // add r4, r4, r5
        process_instruction(&mut emulator, 0b1110_00_0_0100_1_0100_0100_00000000_0101);
    }

    // Assert that the adding completed correctly
    assert_eq!(emulator.cpu.get_register_value(r3), 10);
    assert_eq!(emulator.cpu.get_register_value(r4), 30);
}

#[test]
fn decode_and() {
    assert_eq!(decode_instruction(0x0_00_000_0_0) as usize, and as usize);
}

#[test]
fn decode_b() {
    assert_eq!(decode_instruction(0x0_a0_000_0_0) as usize, b as usize);
}

#[test]
fn behavior_b() {
    let mut emulator = Emulator::dummy();

    // Set the pc to a known value
    let starting_position = 0x0100_0000;
    emulator.cpu.set_register_value(r15, starting_position);

    // Branch with distance of 0
    process_instruction(&mut emulator, 0b1110_101_0_0000_0000_0000_0000_0000_0000);
    assert_eq!(emulator.cpu.get_register_value(r15), starting_position);

    // Branch with largest positive number (0x7fffff<<2)
    process_instruction(&mut emulator, 0b1110_101_0_0111_1111_1111_1111_1111_1111);
    assert_eq!(
        emulator.cpu.get_register_value(r15),
        starting_position + (0x7fffff << 2)
    );

    // Branch with smallest negative number (-4)
    process_instruction(&mut emulator, 0b1110_101_0_1111_1111_1111_1111_1111_1111);
    assert_eq!(
        emulator.cpu.get_register_value(r15),
        starting_position + (0x7fffff << 2) - 4
    );

    // Branch with largest negative number (0x800000<<2)
    process_instruction(&mut emulator, 0b1110_101_0_1000_0000_0000_0000_0000_0000);
    assert_eq!(emulator.cpu.get_register_value(r15), starting_position - 8);
}

#[test]
fn decode_bic() {
    assert_eq!(decode_instruction(0x0_1c_000_0_0) as usize, bic as usize);
}

#[test]
fn behavior_bic() {
    {
        let mut emulator = Emulator::dummy();

        // 0x0300_0001 & !0x0 == 0x0300_0001
        emulator.cpu.set_register_value(r1, 0x0300_0001);
        emulator.cpu.set_register_value(r2, 0x0);

        //   cond    P UBWL Rn   Rd   offset12
        // 0x1110_0011_1101_0001_0000_0000_0000_0001 - bics r0,r1,r2
        process_instruction(&mut emulator, 0xE1D1_0002);

        assert_eq!(emulator.cpu.get_register_value(r1), 0x0300_0001);
        assert_eq!(emulator.cpu.get_register_value(r0), 0x0300_0001);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xFFFF_FFFF & !0x7FFF_FFFF == 0x8000_0000
        // Also check if overflow flags remains unaffected
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x7FFF_FFFF);
        emulator.cpu.set_nzcv(false, false, false, true);

        //   cond    P UBWL Rn   Rd   offset12
        // 0x1110_0011_1101_0001_0000_0000_0000_0001 - bics r0,r1,r2
        process_instruction(&mut emulator, 0xE1D1_0002);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r0), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xAAAA_AAAA & !0xFFFF_FFFF == 0x0
        // Also check if overflow flags remains unaffected
        emulator.cpu.set_register_value(r1, 0xAAAA_AAAA);
        emulator.cpu.set_register_value(r2, 0xFFFF_FFFF);
        emulator.cpu.set_nzcv(false, false, false, true);

        //   cond    P UBWL Rn   Rd   offset12
        // 0x1110_0011_1101_0001_0000_0000_0000_0001 - bics r0,r1,r2
        process_instruction(&mut emulator, 0xE1D1_0002);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xAAAA_AAAA);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF & !0x0000_0007 == 0x3FFF_FFFF
        // Also check if overflow flags remains unaffected
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x0000_0007);
        emulator.cpu.set_nzcv(false, false, false, false);

        //   cond    P UBWL Rn   Rd   offset12
        // 0x1110_0011_1101_0001_0000_1111_0000_0001 - bics r0,r1,r2,lsl 0x1E
        process_instruction(&mut emulator, 0xE1D1_0F02);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x3FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }
}

#[test]
fn decode_bl() {
    assert_eq!(decode_instruction(0x0_b0_000_0_0) as usize, bl as usize);
}

#[test]
fn decode_bx() {
    assert_eq!(decode_instruction(0x0_12_000_1_0) as usize, bx as usize);
}

#[test]
fn behavior_bx() {
    //   cond           SBO  SBO  SBO       Rm
    // 0x1110_0001_0010_1111_1111_1111_0001_0000 - bx r0
    let instruction = 0xE12F_FF10;

    {
        let mut emulator = Emulator::dummy();

        // r0 = 0x0300_0004
        emulator.cpu.set_register_value(r0, 0x0300_0004);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0300_0004);
        assert_eq!(emulator.cpu.get_register_value(r15), 0x0300_0004);
        assert_eq!(emulator.cpu.get_thumb_bit(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // r0 = 0x0300_0005
        emulator.cpu.set_register_value(r0, 0x0300_0005);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0300_0005);
        assert_eq!(emulator.cpu.get_register_value(r15), 0x0300_0004);
        assert_eq!(emulator.cpu.get_thumb_bit(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // r0 = 0x0300_0003 (non-word aligned jump to an address in thumb mode)
        emulator.cpu.set_register_value(r0, 0x0300_0003);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0300_0003);
        assert_eq!(emulator.cpu.get_register_value(r15), 0x0300_0002);
        assert_eq!(emulator.cpu.get_thumb_bit(), true);
    }
}

#[test]
fn decode_cdp() {
    assert_eq!(decode_instruction(0x0_e0_000_0_0) as usize, cdp as usize);
}

#[test]
fn decode_cmn() {
    assert_eq!(decode_instruction(0x0_17_000_0_0) as usize, cmn as usize);
}

#[test]
fn behavior_cmn() {
    //   cond    P UBWL Rn   SBZ  offset12
    // 0x1110_0001_0101_0001_0000_0000_0000_0010 - cmn r1,r2
    let instruction = 0xE171_0002;

    {
        let mut emulator = Emulator::dummy();

        // 0x0, 0x0
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x0);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0, 0x1
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF, 0x1
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r1), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF, 0x7FFF_FFFF
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x7FFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r1), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF, 0x8000_0000
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x8000_0000);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r1), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000, 0x8000_0000
        emulator.cpu.set_register_value(r1, 0x8000_0000);
        emulator.cpu.set_register_value(r2, 0x8000_0000);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xFFFF_FFFF, 0x1
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xFFFF_FFFF, 0x2
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x2);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x2);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }
}

#[test]
fn decode_cmp() {
    assert_eq!(decode_instruction(0x0_15_000_0_0) as usize, cmp as usize);
}

#[test]
fn behavior_cmp() {
    //   cond    P UBWL Rn   SBZ  offset12
    // 0x1110_0001_0101_0001_0000_0000_0000_0010 - cmp r1,r2
    let instruction = 0xE151_0002;

    {
        let mut emulator = Emulator::dummy();

        // 0x0, 0x0
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x0);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0, 0x1
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF, 0x1
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r1), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF, 0x7FFF_FFFF
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x7FFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r1), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF, 0x8000_0000
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x8000_0000);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r1), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000, 0x1
        emulator.cpu.set_register_value(r1, 0x8000_0000);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000, 0x7FFF_FFFF
        emulator.cpu.set_register_value(r1, 0x8000_0000);
        emulator.cpu.set_register_value(r2, 0x7FFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xFFFF_FFFF, 0x1
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xFFFF_FFFF, 0x7FFF_FFFF
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x7FFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xFFFF_FFFF, 0x8000_0000
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x8000_0000);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }
}

#[test]
fn decode_eor() {
    assert_eq!(decode_instruction(0x0_02_000_0_0) as usize, eor as usize);
}

#[test]
fn behavior_eor() {
    //   cond    P UBWL Rn   Rd   offset12
    // 0x1110_0000_0011_0001_0000_0000_0000_0010 - eor r0,r1,r2
    let instruction = 0xE031_0002;

    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xAAAA_AAAA);
        emulator.cpu.set_register_value(r2, 0xBEBE_BEBE);
        emulator.cpu.set_nzcv(false, false, false, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1414_1414);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xAAAA_AAAA);
        assert_eq!(emulator.cpu.get_register_value(r2), 0xBEBE_BEBE);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xAAAA_AAAA);
        emulator.cpu.set_register_value(r2, 0xBEBE_BEBE);
        emulator.cpu.set_nzcv(false, false, true, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1414_1414);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xAAAA_AAAA);
        assert_eq!(emulator.cpu.get_register_value(r2), 0xBEBE_BEBE);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xAAAA_AAAA);
        emulator.cpu.set_register_value(r2, 0xBEBE_BEBE);
        emulator.cpu.set_nzcv(false, false, false, true);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1414_1414);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xAAAA_AAAA);
        assert_eq!(emulator.cpu.get_register_value(r2), 0xBEBE_BEBE);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xAAAA_AAAA);
        emulator.cpu.set_register_value(r2, 0x5555_5555);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xAAAA_AAAA);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x5555_5555);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_register_value(r1, 0xAAAA_AAAA);
        emulator.cpu.set_register_value(r2, 0xAAAA_AAAA);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xAAAA_AAAA);
        assert_eq!(emulator.cpu.get_register_value(r2), 0xAAAA_AAAA);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }
}

#[test]
fn decode_ldc() {
    assert_eq!(decode_instruction(0x0_c1_000_0_0) as usize, ldc as usize);
}

#[test]
fn decode_ldm() {
    // Even though this instruction has multiple modes, they should all overlap
    assert_eq!(decode_instruction(0x0_81_000_0_0) as usize, ldm as usize);
}

#[test]
fn decode_ldr() {
    assert_eq!(decode_instruction(0x0_41_000_0_0) as usize, ldr as usize);
}

#[test]
fn behavior_ldr() {
    // Offset
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0004, 0xaabb_ccdd);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0101_1001_0010_0001_0000_0000_0100 - ldr r1,[r2,0x4]
        process_instruction(&mut emulator, 0xE592_1004);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xaabb_ccdd);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0000);
    }

    // Post-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0000, 0xeeff_1122);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0100_1001_0010_0001_0000_0000_1010 - ldr r1,[r2],0xA
        process_instruction(&mut emulator, 0xE492_100A);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xeeff_1122);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_000A);
    }

    // Pre-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0010, 0x3344_5566);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0101_1011_0010_0001_0000_0001_0000 - ldr r1,[r2,0x10]!
        process_instruction(&mut emulator, 0xE5B2_1010);

        assert_eq!(emulator.memory.read_word(0x0300_0010), 0x3344_5566);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0010);
    }

    // Non word-aligned address
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0004, 0xaabb_ccdd);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0101_1001_0010_0001_0000_0000_0101 - ldr r1,[r2,0x5]
        process_instruction(&mut emulator, 0xE592_1005);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xddaa_bbcc);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0000);
    }

    // Rd == r15
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0014, 0x0400_0083);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0101_1001_0010_1111_0000_0001_0100 - ldr r15,[r2,0x14]
        process_instruction(&mut emulator, 0xE592_F014);

        assert_eq!(emulator.cpu.get_register_value(r15), 0x0400_0080);
    }
}

#[test]
fn decode_ldrb() {
    assert_eq!(decode_instruction(0x0_45_000_0_0) as usize, ldrb as usize);
}

#[test]
fn behavior_ldrb() {
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0008, 0xaabb_ccdd);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0101_1101_0010_0001_0000_0000_1010 - ldrb r1,[r2,0x00A]
        process_instruction(&mut emulator, 0xE5D2_100A);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xbb);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0000);
    }

    // Post-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0000, 0xaabb_ccdd);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0100_1101_0010_0001_0000_0000_0101 - ldrb r1,[r2],0x5
        process_instruction(&mut emulator, 0xE4D2_1005);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xdd);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0005);
    }

    // Pre-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0010, 0xaabb_ccdd);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0101_1111_0010_0001_0000_0001_0000 - ldrb r1,[r2,0x10]!
        process_instruction(&mut emulator, 0xE5F2_1010);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xdd);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0010);
    }
}

#[test]
fn decode_ldrbt() {
    assert_eq!(decode_instruction(0x0_47_000_0_0) as usize, ldrbt as usize);
}

#[test]
fn behavior_ldrbt() {
    // Post-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0000, 0xaabb_ccdd);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0100_1111_0010_0001_0000_0000_0101 - ldrbt r1,[r2],0x5
        process_instruction(&mut emulator, 0xE4F2_1005);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xdd);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0005);
    }
}

#[test]
fn decode_ldrh() {
    assert_eq!(decode_instruction(0x0_05_000_b_0) as usize, ldrh as usize);
}

#[test]
fn behavior_ldrh() {
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0004, 0xaaaa_bbcc);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   addr      addr
        // 0b1110_0001_1101_0010_0001_0000_1011_0100 - ldrh r1,[r2,0x4]
        process_instruction(&mut emulator, 0xE1D2_10B4);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xbbcc);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0000);
    }

    // Pre-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0004, 0xaaaa_bbcc);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   addr      addr
        // 0b1110_0001_1111_0010_0001_0000_1011_0100 - ldrh r1,[r2,0x4]!
        process_instruction(&mut emulator, 0xE1F2_10B4);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xbbcc);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0004);
    }

    // Post-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0000, 0xaaaa_bbcc);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   addr      addr
        // 0b1110_0000_1101_0010_0001_0000_1011_0100 - ldrh r1,[r2],0x4
        process_instruction(&mut emulator, 0xE0D2_10B4);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xbbcc);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0004);
    }
}

#[test]
fn decode_ldrsb() {
    assert_eq!(decode_instruction(0x0_05_000_d_0) as usize, ldrsb as usize);
}

#[test]
fn behavior_ldrsb() {
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0004, 0xaaaa_bbcc);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   addr      addr
        // 0b1110_0001_1101_0010_0001_0000_1101_0100 - ldrsb r1,[r2,0x4]
        process_instruction(&mut emulator, 0xE1D2_10D4);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xffff_ffcc);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0000);
    }

    // Pre-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0004, 0xaaaa_bbcc);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   addr      addr
        // 0b1110_0001_1111_0010_0001_0000_1101_0100 - ldrsb r1,[r2,0x4]!
        process_instruction(&mut emulator, 0xE1F2_10D4);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xffff_ffcc);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0004);
    }

    // Post-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0000, 0xaaaa_bbcc);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   addr      addr
        // 0b1110_0000_1101_0010_0001_0000_1101_0100 - ldrsb r1,[r2],0x4
        process_instruction(&mut emulator, 0xE0D2_10D4);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xffff_ffcc);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0004);
    }

    // Offset with a positive byte value
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0004, 0xaaaa_bb7f);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   addr      addr
        // 0b1110_0001_1101_0010_0001_0000_1101_0100 - ldrsb r1,[r2,0x4]
        process_instruction(&mut emulator, 0xE1D2_10D4);

        assert_eq!(emulator.cpu.get_register_value(r1), 0x7f);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0000);
    }
}

#[test]
fn decode_ldrsh() {
    assert_eq!(decode_instruction(0x0_05_000_f_0) as usize, ldrsh as usize);
}

#[test]
fn behavior_ldrsh() {
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0004, 0xaaaa_bbcc);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   addr      addr
        // 0b1110_0001_1101_0010_0001_0000_1111_0100 - ldrsh r1,[r2,0x4]
        process_instruction(&mut emulator, 0xE1D2_10F4);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xffff_bbcc);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0000);
    }

    // Pre-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0004, 0xaaaa_bbcc);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   addr      addr
        // 0b1110_0001_1111_0010_0001_0000_1111_0100 - ldrsh r1,[r2,0x4]!
        process_instruction(&mut emulator, 0xE1F2_10F4);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xffff_bbcc);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0004);
    }

    // Post-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0000, 0xaaaa_bbcc);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   addr      addr
        // 0b1110_0000_1101_0010_0001_0000_1111_0100 - ldrsh r1,[r2],0x4
        process_instruction(&mut emulator, 0xE0D2_10F4);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xffff_bbcc);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0004);
    }

    // Offset with a positive byte value
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0004, 0xaaaa_7fff);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   addr      addr
        // 0b1110_0001_1101_0010_0001_0000_1111_0100 - ldrsh r1,[r2,0x4]
        process_instruction(&mut emulator, 0xE1D2_10F4);

        assert_eq!(emulator.cpu.get_register_value(r1), 0x7fff);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0000);
    }
}

#[test]
fn decode_ldrt() {
    assert_eq!(decode_instruction(0x0_43_000_0_0) as usize, ldrt as usize);
}

#[test]
fn behavior_ldrt() {
    // Post-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0000, 0xeeff_1122);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0100_1011_0010_0001_0000_0000_1010 - ldrt r1,[r2],0xA
        process_instruction(&mut emulator, 0xE4B2_100A);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xeeff_1122);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_000A);
    }

    // Non word-aligned address
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0000, 0xaabb_ccdd);
        emulator.cpu.set_register_value(r2, 0x0300_0001);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0100_1011_0010_0001_0000_0000_0101 - ldrt r1,[r2],0x5
        process_instruction(&mut emulator, 0xE4B2_1005);

        assert_eq!(emulator.cpu.get_register_value(r1), 0xddaa_bbcc);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0006);
    }
}

#[test]
fn decode_mcr() {
    assert_eq!(decode_instruction(0x0_e0_000_1_0) as usize, mcr as usize);
}

#[test]
fn decode_mla() {
    assert_eq!(decode_instruction(0x0_02_000_9_0) as usize, mla as usize);
}

#[test]
fn behavior_mla() {
    //   cond         S Rd   Rn   Rs        Rm
    // 0x1110_0000_0011_0000_0011_0010_1001_0001 - mlas r0,r1,r2,r3
    let instruction = 0xE030_3291;

    {
        let mut emulator = Emulator::dummy();

        // Rm    Rs    Rn
        // r1    r2    r3
        // 0x0 * 0x0 + 0x0
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_register_value(r3, 0x0);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 * 0x0 + 0x1
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_register_value(r3, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF * 0x1 + 0x0
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x1);
        emulator.cpu.set_register_value(r3, 0x0);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x1 * 0x7FFF_FFFF + 0x2
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x1);
        emulator.cpu.set_register_value(r3, 0x2);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x8000_0001);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x2);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xAAAA_AAAA * 0x5555_5555 + 0x2
        emulator.cpu.set_register_value(r1, 0xAAAA_AAAA);
        emulator.cpu.set_register_value(r2, 0x5555_5555);
        emulator.cpu.set_register_value(r3, 0x2);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x71C7_1C74);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xAAAA_AAAA);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x5555_5555);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x2);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xAAAA_AAAA * 0x5555_5555 + 0x1000_0000
        emulator.cpu.set_register_value(r1, 0xAAAA_AAAA);
        emulator.cpu.set_register_value(r2, 0x5555_5555);
        emulator.cpu.set_register_value(r3, 0x1000_0000);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x81C7_1C72);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xAAAA_AAAA);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x5555_5555);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x1000_0000);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xAAAA_AAAA * 0x5555_5555 + 0x8FFF_FFFF
        emulator.cpu.set_register_value(r1, 0xAAAA_AAAA);
        emulator.cpu.set_register_value(r2, 0x5555_5555);
        emulator.cpu.set_register_value(r3, 0x8FFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x01C7_1C71);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xAAAA_AAAA);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x5555_5555);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x8FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }
}

#[test]
fn decode_mov() {
    assert_eq!(decode_instruction(0x0_1a_000_0_0) as usize, mov as usize);
}

#[test]
fn behavior_mov() {
    //   cond   I     S SBZ  Rd   shift_op
    // 0x1110_0001_1011_0000_0000_0000_0000_0001 - movs r0,r1
    let instruction = 0xE1B0_0001;

    {
        let mut emulator = Emulator::dummy();

        // 0x0
        emulator.cpu.set_register_value(r0, 0xAAAA_AAAA);
        emulator.cpu.set_register_value(r1, 0x0);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 + carry_flag set + overflow_flag set
        emulator.cpu.set_register_value(r0, 0xAAAA_AAAA);
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_nzcv(false, false, true, true);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x1
        emulator.cpu.set_register_value(r0, 0xAAAA_AAAA);
        emulator.cpu.set_register_value(r1, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000
        emulator.cpu.set_register_value(r0, 0xAAAA_AAAA);
        emulator.cpu.set_register_value(r1, 0x8000_0000);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }
}

#[test]
fn decode_mrc() {
    assert_eq!(decode_instruction(0x0_e1_000_1_0) as usize, mrc as usize);
}

#[test]
fn decode_mrs() {
    assert_eq!(decode_instruction(0x0_10_000_0_0) as usize, mrs as usize);
}

#[test]
fn behavior_mrs() {
    //   cond       R   SBO  Rd   SBZ
    // 0x1110_0001_0000_1111_0000_0000_0000_0000 - mrs r0,cpsr
    let instruction = 0xE10F_0000;

    {
        let mut emulator = Emulator::dummy();

        emulator.cpu.set_register_value(cpsr, 0xaabb_cc10);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xaabb_cc10);
    }

    //   cond       R   SBO  Rd   SBZ
    // 0x1110_0001_0100_1111_0000_0000_0000_0000 - mrs r0,spsr
    let instruction = 0xE14F_0000;

    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.set_operation_mode(OperationModes::SVC);

        emulator.cpu.set_register_value(spsr, 0xaabb_cc12);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xaabb_cc12);
    }
}

#[test]
fn decode_msr() {
    assert_eq!(decode_instruction(0x0_12_000_0_0) as usize, msr as usize);
}

#[test]
fn decode_mul() {
    assert_eq!(decode_instruction(0x0_00_000_9_0) as usize, mul as usize);
}

#[test]
fn behavior_mul() {
    //   cond SBZ  SBZS Rd   SBZ  Rs        Rm
    // 0x1110_0000_0001_0000_0000_0010_1001_0001 - muls r0,r1,r2
    let instruction = 0xE010_0291;

    {
        let mut emulator = Emulator::dummy();

        // 0x0 * 0x0 == 0x0
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x0);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 * 0x1 == 0x0
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x1 * 0x1 == 0x1
        emulator.cpu.set_register_value(r1, 0x1);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x1 * 0x1 == 0x1 (carry_flag & overflow_flag set)
        emulator.cpu.set_register_value(r1, 0x1);
        emulator.cpu.set_register_value(r2, 0x1);
        emulator.cpu.set_nzcv(false, false, true, true);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x4000_0000 * 0x2 == 0x8000_0000
        emulator.cpu.set_register_value(r1, 0x4000_0000);
        emulator.cpu.set_register_value(r2, 0x2);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x2);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 * 0x2 == 0x0
        emulator.cpu.set_register_value(r1, 0x8000_0000);
        emulator.cpu.set_register_value(r2, 0x2);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x2);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xFFFF_FFFF * 0xFFFF_FFFF == 0x1
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r2, 0xFFFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF * 0x7FFF_FFFF == 0x1
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x7FFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7F0F_0F0F * 0x7F0F_0F0F == 0x1
        emulator.cpu.set_register_value(r1, 0x7F0F_0F0F);
        emulator.cpu.set_register_value(r2, 0x7F0F_0F0F);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xA6A4_C2E1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x7F0F_0F0F);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x7F0F_0F0F);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }
}

#[test]
fn decode_mvn() {
    assert_eq!(decode_instruction(0x0_1e_000_0_0) as usize, mvn as usize);
}

#[test]
fn behavior_mvn() {
    //   cond   I     S SBZ  Rd   shift_op
    // 0x1110_0001_1111_0000_0000_0000_0000_0001 - movs r0,r1
    let instruction = 0xE1F0_0001;

    {
        let mut emulator = Emulator::dummy();

        // 0x0
        emulator.cpu.set_register_value(r0, 0xAAAA_AAAA);
        emulator.cpu.set_register_value(r1, 0x0);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 + carry_flag set + overflow_flag set
        emulator.cpu.set_register_value(r0, 0xAAAA_AAAA);
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_nzcv(false, false, true, true);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x1
        emulator.cpu.set_register_value(r0, 0xAAAA_AAAA);
        emulator.cpu.set_register_value(r1, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xFFFF_FFFE);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x1);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000
        emulator.cpu.set_register_value(r0, 0xAAAA_AAAA);
        emulator.cpu.set_register_value(r1, 0x8000_0000);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }
}

#[test]
fn decode_or() {
    assert_eq!(decode_instruction(0x0_18_000_0_0) as usize, or as usize);
}

#[test]
fn behavior_or() {
    //   cond   I_opc_S Rn   Rd   shift_op
    // 0x1110_0001_1001_0001_0000_0000_0000_0001 - orrs r0,r1,r2
    let instruction = 0xE191_0002;

    {
        let mut emulator = Emulator::dummy();

        // 0x0 | 0x0
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x0);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 | 0x0 (overflow_flag set)
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_nzcv(false, false, false, true);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 | 0x1
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xAAAA_AAAA | 0x5555_5555
        emulator.cpu.set_register_value(r1, 0xAAAA_AAAA);
        emulator.cpu.set_register_value(r2, 0x5555_5555);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xAAAA_AAAA);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x5555_5555);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }
}

#[test]
fn decode_rsb() {
    assert_eq!(decode_instruction(0x0_06_000_0_0) as usize, rsb as usize);
}

#[test]
fn behavior_rsb() {
    //   cond   I_opc_S Rn   Rd   offset12
    // 0x1110_0000_0111_0001_0000_0000_0000_0010 - rsbs r0,r1,r2
    let instruction = 0xE071_0002;

    {
        let mut emulator = Emulator::dummy();

        // 0x1 - 0x1 = 0x0
        emulator.cpu.set_register_value(r1, 0x1);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x2 - 0x1 = 0x1
        emulator.cpu.set_register_value(r1, 0x1);
        emulator.cpu.set_register_value(r2, 0x2);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x2);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 - 0x1 = 0x7FFF_FFFF
        emulator.cpu.set_register_value(r1, 0x1);
        emulator.cpu.set_register_value(r2, 0x8000_0000);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x1 - 0x2 = 0xFFFF_FFFF
        emulator.cpu.set_register_value(r1, 0x2);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x2);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x1 - 0x2 = 0xFFFF_FFFF
        emulator.cpu.set_register_value(r1, 0x2);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x2);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 - 0x7FFF_FFFF = 0x1
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x8000_0000);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF - 0xFFFF_FFFF
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x7FFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), true);
    }
}

#[test]
fn decode_rsc() {
    assert_eq!(decode_instruction(0x0_0e_000_0_0) as usize, rsc as usize);
}

#[test]
fn behavior_rsc() {
    //   cond   I_opc_S Rn   Rd   shift_op
    // 0x1110_0000_1111_0001_0000_0000_0000_0010 - sbcs r0,r1,r2
    let instruction = 0xE0F1_0002;

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 - 0x1 + c_flag == 0x7FFF_FFFF
        emulator.cpu.set_register_value(r1, 0x1);
        emulator.cpu.set_register_value(r2, 0x8000_0000);
        emulator.cpu.set_nzcv(false, false, true, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 - 0x1 == 0x7FFF_FFFE
        emulator.cpu.set_register_value(r1, 0x1);
        emulator.cpu.set_register_value(r2, 0x8000_0000);
        emulator.cpu.set_nzcv(false, false, false, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x7FFF_FFFE);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 - 0x0 + c_flag == 0x0
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_nzcv(false, false, true, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 - 0x0 == 0xFFFF_FFFF
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_nzcv(false, false, false, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 - 0x1 == 0xFFFF_FFFE
        emulator.cpu.set_register_value(r1, 0x1);
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_nzcv(false, false, false, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xFFFF_FFFE);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF - 0xFFFF_FFFF + c_flag == 0x8000_0000
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x7FFF_FFFF);
        emulator.cpu.set_nzcv(false, false, true, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 - 0x7FFF_FFFF + c_flag == 0x1
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0x8000_0000);
        emulator.cpu.set_nzcv(false, false, true, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 - 0x8000_0000 + c_flag == 0x0
        emulator.cpu.set_register_value(r1, 0x8000_0000);
        emulator.cpu.set_register_value(r2, 0x8000_0000);
        emulator.cpu.set_nzcv(false, false, true, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 - 0x8000_0000 == 0xFFFF_FFFF
        emulator.cpu.set_register_value(r1, 0x8000_0000);
        emulator.cpu.set_register_value(r2, 0x8000_0000);
        emulator.cpu.set_nzcv(false, false, false, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }
}

#[test]
fn decode_sbc() {
    assert_eq!(decode_instruction(0x0_0c_000_0_0) as usize, sbc as usize);
}

#[test]
fn behavior_sbc() {
    //   cond    P UBWL Rn   Rd   offset12
    // 0x1110_0000_1101_0001_0000_0000_0000_0010 - sbcs r0,r1,r2
    let instruction = 0xE0D1_0002;

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 - 0x1 + c_flag == 0x7FFF_FFFF
        emulator.cpu.set_register_value(r1, 0x8000_0000);
        emulator.cpu.set_register_value(r2, 0x1);
        emulator.cpu.set_nzcv(false, false, true, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 - 0x1 == 0x7FFF_FFFE
        emulator.cpu.set_register_value(r1, 0x8000_0000);
        emulator.cpu.set_register_value(r2, 0x1);
        emulator.cpu.set_nzcv(false, false, false, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x7FFF_FFFE);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 - 0x0 + c_flag == 0x0
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_nzcv(false, false, true, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 - 0x0 == 0xFFFF_FFFF
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_nzcv(false, false, false, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 - 0x1 == 0xFFFF_FFFE
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x1);
        emulator.cpu.set_nzcv(false, false, false, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xFFFF_FFFE);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF - 0xFFFF_FFFF + c_flag == 0x8000_0000
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0xFFFF_FFFF);
        emulator.cpu.set_nzcv(false, false, true, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 - 0x7FFF_FFFF + c_flag == 0x1
        emulator.cpu.set_register_value(r1, 0x8000_0000);
        emulator.cpu.set_register_value(r2, 0x7FFF_FFFF);
        emulator.cpu.set_nzcv(false, false, true, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 - 0x8000_0000 + c_flag == 0x0
        emulator.cpu.set_register_value(r1, 0x8000_0000);
        emulator.cpu.set_register_value(r2, 0x8000_0000);
        emulator.cpu.set_nzcv(false, false, true, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 - 0x8000_0000 == 0xFFFF_FFFF
        emulator.cpu.set_register_value(r1, 0x8000_0000);
        emulator.cpu.set_register_value(r2, 0x8000_0000);
        emulator.cpu.set_nzcv(false, false, false, false);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }
}

#[test]
fn decode_smlal() {
    assert_eq!(decode_instruction(0x0_0e_000_9_0) as usize, smlal as usize);
}

#[test]
fn behavior_smlal() {
    //   cond         S RdHi RdLo Rs        Rm
    // 0x1110_0000_1111_0001_0000_0011_1001_0010 - smlals r0,r1,r2,r3
    let instruction = 0xE0F1_0392;

    {
        let mut emulator = Emulator::dummy();

        // 0x0 * 0x0 == 0x0
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_register_value(r3, 0x0);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 * 0x0 == 0x0 (carry_flag and overflow_flag set)
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_register_value(r3, 0x0);
        emulator.cpu.set_nzcv(false, false, true, true);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 * 0x0 == (0x0 + 0x1, 0x0 + 0x1)
        emulator.cpu.set_register_value(r0, 0x1);
        emulator.cpu.set_register_value(r1, 0x1);
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_register_value(r3, 0x0);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x4000_0001 * 0x2 == (0x8000_0002 + 0x8000_0000, 0x0 + carry(0x1))
        emulator.cpu.set_register_value(r0, 0x8000_0000);
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x4000_0001);
        emulator.cpu.set_register_value(r3, 0x2);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x2);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x4000_0001);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x2);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xFFFF_FFFF * 0xFFFF_FFFF == (0x1, 0x0)
        emulator.cpu.set_register_value(r2, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r3, 0xFFFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r3), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF * 0x7FFF_FFFF == (0x1, 0x3FFF_FFFF)
        emulator.cpu.set_register_value(r2, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r3, 0x7FFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x3FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 * 0x8000_0000 == (0x0, 0x4000_0000 + 0x4000_0000)
        emulator.cpu.set_register_value(r0, 0x0);
        emulator.cpu.set_register_value(r1, 0x4000_0000);
        emulator.cpu.set_register_value(r2, 0x8000_0000);
        emulator.cpu.set_register_value(r3, 0x8000_0000);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }
}

#[test]
fn decode_smull() {
    assert_eq!(decode_instruction(0x0_0c_000_9_0) as usize, smull as usize);
}

#[test]
fn behavior_smull() {
    //   cond         S RdHi RdLo Rs        Rm
    // 0x1110_0000_1101_0001_0000_0011_1001_0010 - smulls r0,r1,r2,r3
    let instruction = 0xE0D1_0392;

    {
        let mut emulator = Emulator::dummy();

        // 0x0 * 0x0 == 0x0
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_register_value(r3, 0x0);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 * 0x0 == 0x0 (carry_flag and overflow_flag set)
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_register_value(r3, 0x0);
        emulator.cpu.set_nzcv(false, false, true, true);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x1 * 0x1 == 0x1
        emulator.cpu.set_register_value(r2, 0x1);
        emulator.cpu.set_register_value(r3, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF * 0x7FFF_FFFF == (0x1, 0x3FFF_FFFF)
        emulator.cpu.set_register_value(r2, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r3, 0x7FFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x3FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 * 0x7FFF_FFFF == (0x8000_0000, 0xC000_0000)
        emulator.cpu.set_register_value(r2, 0x8000_0000);
        emulator.cpu.set_register_value(r3, 0x7FFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xC000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xFFFF_FFFF * 0xFFFF_FFFF == (0x1, 0x0)
        emulator.cpu.set_register_value(r2, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r3, 0xFFFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r3), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }
}

#[test]
fn decode_stc() {
    assert_eq!(decode_instruction(0x0_c0_000_0_0) as usize, stc as usize);
}

#[test]
fn decode_stm() {
    // Even though this instruction has multiple modes, they should all overlap
    assert_eq!(decode_instruction(0x0_80_000_0_0) as usize, stm as usize);
}

#[test]
fn decode_str() {
    assert_eq!(decode_instruction(0x0_40_000_0_0) as usize, str as usize);
}

#[test]
fn behavior_str() {
    // Offset
    {
        let mut emulator = Emulator::dummy();

        emulator.cpu.set_register_value(r1, 0xaaaa_aaaa);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0101_1000_0010_0001_0000_0000_1011 - str r1,[r2,0x00B]
        process_instruction(&mut emulator, 0xE582_100B);

        assert_eq!(emulator.memory.read_word(0x0300_0008), 0xaaaa_aaaa);
    }

    // Pre-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.cpu.set_register_value(r1, 0xcccc_cccc);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0101_1010_0010_0001_0000_0000_1011 - str r1,[r2,0x00B]!
        process_instruction(&mut emulator, 0xE5A2_100B);

        assert_eq!(emulator.memory.read_word(0x0300_0008), 0xcccc_cccc);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_000B);
    }

    // Post-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.cpu.set_register_value(r1, 0xbbbb_bbbb);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0100_1000_0010_0001_0000_0000_1011 - str r1,[r2],0x00B
        process_instruction(&mut emulator, 0xE482_100B);

        assert_eq!(emulator.memory.read_word(0x0300_0000), 0xbbbb_bbbb);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_000B);
    }
}

#[test]
fn decode_strb() {
    assert_eq!(decode_instruction(0x0_44_000_0_0) as usize, strb as usize);
}

#[test]
fn behavior_strb() {
    // Offset
    {
        let mut emulator = Emulator::dummy();

        emulator.cpu.set_register_value(r1, 0xffff_ffaa);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0101_1100_0010_0001_0000_0000_1010 - strb r1,[r2,0x00A]
        process_instruction(&mut emulator, 0xE5C2_100A);

        assert_eq!(emulator.memory.read_byte(0x0300_0009), 0x0);
        assert_eq!(emulator.memory.read_byte(0x0300_000A), 0xaa);
        assert_eq!(emulator.memory.read_byte(0x0300_000B), 0x0);

        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0000);
    }

    // Post-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.cpu.set_register_value(r1, 0xffff_ffbb);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0100_1100_0010_0001_0000_0000_0101 - strb r1,[r2],0x5
        process_instruction(&mut emulator, 0xE4C2_1005);

        assert_eq!(emulator.memory.read_byte(0x0300_0000), 0xbb);
        assert_eq!(emulator.memory.read_byte(0x0300_0001), 0x0);

        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0005);
    }

    // Pre-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.cpu.set_register_value(r1, 0xffff_ffcc);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0101_1110_0010_0001_0000_0001_0000 - strb r1,[r2,0x10]!
        process_instruction(&mut emulator, 0xE5E2_1010);

        assert_eq!(emulator.memory.read_byte(0x0300_000F), 0x0);
        assert_eq!(emulator.memory.read_byte(0x0300_0010), 0xcc);
        assert_eq!(emulator.memory.read_byte(0x0300_0011), 0x0);

        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0010);
    }
}

#[test]
fn decode_strbt() {
    assert_eq!(decode_instruction(0x0_46_000_0_0) as usize, strbt as usize);
}

#[test]
fn behavior_strbt() {
    // Post-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.cpu.set_register_value(r1, 0xffff_ffbb);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0100_1110_0010_0001_0000_0000_0101 - strbt r1,[r2],0x5
        process_instruction(&mut emulator, 0xE4E2_1005);

        assert_eq!(emulator.memory.read_byte(0x0300_0000), 0xbb);
        assert_eq!(emulator.memory.read_byte(0x0300_0001), 0x0);

        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0005);
    }
}

#[test]
fn decode_strh() {
    assert_eq!(decode_instruction(0x0_00_000_b_0) as usize, strh as usize);
}

#[test]
fn behavior_strh() {
    {
        let mut emulator = Emulator::dummy();

        emulator.cpu.set_register_value(r1, 0xaaaa_bbcc);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   addr      addr
        // 0b1110_0001_1100_0010_0001_0000_1011_0100 - strh r1,[r2,0x4]
        process_instruction(&mut emulator, 0xE1C2_10B4);
        assert_eq!(emulator.memory.read_half_word(0x0300_0004), 0xbbcc);
    }

    // Pre-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.cpu.set_register_value(r1, 0xaaaa_bbcc);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   addr      addr
        // 0b1110_0001_1110_0010_0001_0000_1011_0100 - strh r1,[r2,0x4]!
        process_instruction(&mut emulator, 0xE1E2_10B4);

        assert_eq!(emulator.memory.read_half_word(0x0300_0004), 0xbbcc);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0004);
    }

    // Post-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.cpu.set_register_value(r1, 0xaaaa_bbcc);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   addr      addr
        // 0b1110_0000_1100_0010_0001_0000_1011_0100 - strh r1,[r2],0x4
        process_instruction(&mut emulator, 0xE0C2_10B4);

        assert_eq!(emulator.memory.read_half_word(0x0300_0000), 0xbbcc);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0004);
    }
}

#[test]
fn decode_strt() {
    assert_eq!(decode_instruction(0x0_42_000_0_0) as usize, strt as usize);
}

#[test]
fn behavior_strt() {
    // Post-indexed
    {
        let mut emulator = Emulator::dummy();

        emulator.cpu.set_register_value(r1, 0xbbbb_bbbb);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0100_1010_0010_0001_0000_0000_1011 - strt r1,[r2],0xB
        process_instruction(&mut emulator, 0xE4A2_100B);

        assert_eq!(emulator.memory.read_word(0x0300_0000), 0xbbbb_bbbb);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_000B);
    }
}

#[test]
fn decode_sub() {
    assert_eq!(decode_instruction(0x0_04_000_0_0) as usize, sub as usize);
}

#[test]
fn behavior_sub() {
    let mut emulator = Emulator::dummy();

    //   cond    P UBWL Rn   Rd   offset12
    // 0x1110_0000_0101_0001_0000_0000_0000_0010 - subs r0,r1,r2
    let instruction = 0xE051_0002;

    // r1 = 0xFFFF_FFFE
    // r2 = 0x7FFF_FFFF
    // r0 = 0x7FFF_FFFF
    // nzcv = 0b0011
    {
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFE);
        emulator.cpu.set_register_value(r2, 0x7FFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    // r1 = 0x1
    // r2 = 0x1
    // r0 = 0x0
    // nzcv = 0b0110
    {
        emulator.cpu.set_register_value(r1, 0x1);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    // r1 = 0x0
    // r2 = 0x1
    // r0 = 0xFFFF_FFFF
    // nzcv = 0b1000
    {
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    // r1 = 0xFFFF_FFFF
    // r2 = 0xFFFF_FFFE
    // r0 = 0x1
    // nzcv = 0b0010
    {
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r2, 0xFFFF_FFFE);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    // r1 = 0x0
    // r2 = 0xFFFF_FFFF
    // r0 = 0x1
    // nzcv = 0b0000
    {
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0xFFFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    // r1 = 0x2
    // r2 = 0x1
    // r0 = 0x1
    // nzcv = 0b0010
    {
        emulator.cpu.set_register_value(r1, 0x2);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    // r1 = 0xFFFF_FFFF
    // r2 = 0xFFFF_FFFF
    // r0 = 0x0
    // nzcv = 0b0110
    {
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r2, 0xFFFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    // r1 = 0xFFFF_FFFE
    // r2 = 0xFFFF_FFFF
    // r0 = 0xFFFF_FFFF
    // nzcv = 0b0110
    {
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFE);
        emulator.cpu.set_register_value(r2, 0xFFFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    // r1 = 0x7FFF_FFFF
    // r2 = 0xFFFF_FFFF
    // r0 = 0x8000_0000
    // nzcv = 0b1001
    {
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0xFFFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), true);
    }
}

#[test]
fn decode_swi() {
    assert_eq!(decode_instruction(0x0_f0_000_0_0) as usize, swi as usize);
}

#[test]
fn behavior_swi() {
    //   cond SBO  immed_24
    // 0x1110_1111_0000_0000_0000_0000_0000_0000 - swi 0h (soft reset?)
    let instruction = 0xEF00_0000;

    // TODO: un-verified behavior
    {
        let mut emulator = Emulator::dummy();
        emulator.cpu.reset();

        emulator.cpu.set_register_value(r15, 0xaabb_ddcc);
        emulator.cpu.set_register_value(cpsr, 0xeeff_9910);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_operation_mode(), Some(OperationModes::SVC));
        assert_eq!(emulator.cpu.registers.r14_svc, 0xaabb_ddd0);
        assert_eq!(emulator.cpu.registers.spsr_svc, 0xeeff_9910);
        assert_eq!(emulator.cpu.is_fiq_disabled(), true);
        assert_eq!(emulator.cpu.is_irq_disabled(), false);
        assert_eq!(emulator.cpu.get_register_value(r15), 0x0000_0008);
    }
}

#[test]
fn decode_swp() {
    assert_eq!(decode_instruction(0x0_10_000_9_0) as usize, swp as usize);
}

#[test]
fn behavior_swp() {
    //   cond           Rn   Rd   SBZ       Rm
    // 0x1110_0001_0000_0010_0000_0000_1001_0001 - swp r0,r1,[r2]
    let instruction = 0xE102_0091;

    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0000, 0xaabb_ccdd);
        emulator.cpu.set_register_value(r1, 0xeeff_0011);
        emulator.cpu.set_register_value(r2, 0x0300_0000);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xaabb_ccdd);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xeeff_0011);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0000);
        assert_eq!(emulator.memory.read_word(0x0300_0000), 0xeeff_0011);
    }

    // With rotate (24 bits)
    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0000, 0xaabb_ccdd);
        emulator.cpu.set_register_value(r1, 0xeeff_0011);
        emulator.cpu.set_register_value(r2, 0x0300_0003);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xbbccddaa);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xeeff_0011);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0003);
        assert_eq!(emulator.memory.read_word(0x0300_0000), 0xeeff_0011);
    }
}

#[test]
fn decode_swpb() {
    assert_eq!(decode_instruction(0x0_14_000_9_0) as usize, swpb as usize);
}

#[test]
fn behavior_swpb() {
    //   cond           Rn   Rd   SBZ       Rm
    // 0x1110_0001_0100_0010_0000_0000_1001_0001 - swp r0,r1,[r2]
    let instruction = 0xE142_0091;

    {
        let mut emulator = Emulator::dummy();

        emulator.memory.write_word(0x0300_0000, 0xaabb_ccdd);
        emulator.cpu.set_register_value(r1, 0xeeff_0011);
        emulator.cpu.set_register_value(r2, 0x0300_0002);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xbb);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xeeff_0011);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0300_0002);
        assert_eq!(emulator.memory.read_word(0x0300_0000), 0xaa11_ccdd);
    }
}

#[test]
fn decode_teq() {
    assert_eq!(decode_instruction(0x0_13_000_0_0) as usize, teq as usize);
}

#[test]
fn behavior_teq() {
    //   cond   I_opc_S Rn   SBZ  shift_op
    // 0x1110_0001_0011_0000_0000_0000_0000_0001 - teq r0,r1
    let instruction = 0xE130_0001;

    {
        let mut emulator = Emulator::dummy();

        // 0x0 ^ 0x0
        emulator.cpu.set_register_value(r0, 0x0);
        emulator.cpu.set_register_value(r1, 0x0);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 ^ 0x0 (carry_flag and overflow_flag set)
        emulator.cpu.set_register_value(r0, 0x0);
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_nzcv(false, false, true, true);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x1 ^ 0x0
        emulator.cpu.set_register_value(r0, 0x1);
        emulator.cpu.set_register_value(r1, 0x0);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xFFFF_FFFF ^ 0x7FFF_FFFF
        emulator.cpu.set_register_value(r0, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 ^ 0x8000_0000
        emulator.cpu.set_register_value(r0, 0x8000_0000);
        emulator.cpu.set_register_value(r1, 0x8000_0000);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }
}

#[test]
fn decode_tst() {
    assert_eq!(decode_instruction(0x0_11_000_0_0) as usize, tst as usize);
}

#[test]
fn behavior_tst() {
    //   cond   I_opc_S Rn   SBZ  shift_op
    // 0x1110_0001_0001_0000_0000_0000_0000_0001 - tst r0,r1
    let instruction = 0xE110_0001;

    {
        let mut emulator = Emulator::dummy();

        // 0x0 & 0x0
        emulator.cpu.set_register_value(r0, 0x0);
        emulator.cpu.set_register_value(r1, 0x0);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 & 0x0 (carry_flag and overflow_flag set)
        // Carry flag is not affected in this case but it might be in other cases, depending on the
        // shifter operand.
        emulator.cpu.set_register_value(r0, 0x0);
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_nzcv(false, false, true, true);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x1 & 0x1
        emulator.cpu.set_register_value(r0, 0x1);
        emulator.cpu.set_register_value(r1, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 & 0x8000_0000
        emulator.cpu.set_register_value(r0, 0x8000_0000);
        emulator.cpu.set_register_value(r1, 0x8000_0000);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF & 0x8000_0000
        emulator.cpu.set_register_value(r0, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r1, 0x8000_0000);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }
}

#[test]
fn decode_umlal() {
    assert_eq!(decode_instruction(0x0_0a_000_9_0) as usize, umlal as usize);
}

#[test]
fn behavior_umlal() {
    //   cond         S RdHi RdLo Rs        Rm
    // 0x1110_0000_1011_0001_0000_0011_1001_0010 - umlals r0,r1,r2,r3
    let instruction = 0xE0B1_0392;

    {
        let mut emulator = Emulator::dummy();

        // 0x0 * 0x0 == 0x0
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_register_value(r3, 0x0);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 * 0x0 == 0x0 (carry_flag and overflow_flag set)
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_register_value(r3, 0x0);
        emulator.cpu.set_nzcv(false, false, true, true);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 * 0x0 == (0x0 + 0x1, 0x0 + 0x1)
        emulator.cpu.set_register_value(r0, 0x1);
        emulator.cpu.set_register_value(r1, 0x1);
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_register_value(r3, 0x0);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x4000_0001 * 0x2 == (0x8000_0002 + 0x8000_0000, 0x0 + carry(0x1))
        emulator.cpu.set_register_value(r0, 0x8000_0000);
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x4000_0001);
        emulator.cpu.set_register_value(r3, 0x2);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x2);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x4000_0001);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x2);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xFFFF_FFFF * 0xFFFF_FFFF == (0x1, 0xFFFF_FFFE)
        emulator.cpu.set_register_value(r2, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r3, 0xFFFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xFFFF_FFFE);
        assert_eq!(emulator.cpu.get_register_value(r2), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r3), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF * 0x7FFF_FFFF == (0x1, 0x3FFF_FFFF)
        emulator.cpu.set_register_value(r2, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r3, 0x7FFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x3FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 * 0x8000_0000 == (0x0, 0x4000_0000 + 0x4000_0000)
        emulator.cpu.set_register_value(r0, 0x0);
        emulator.cpu.set_register_value(r1, 0x4000_0000);
        emulator.cpu.set_register_value(r2, 0x8000_0000);
        emulator.cpu.set_register_value(r3, 0x8000_0000);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }
}

#[test]
fn decode_umull() {
    assert_eq!(decode_instruction(0x0_08_000_9_0) as usize, umull as usize);
}

#[test]
fn behavior_umull() {
    //   cond         S RdHi RdLo Rs        Rm
    // 0x1110_0000_1001_0001_0000_0011_1001_0010 - umulls r0,r1,r2,r3
    let instruction = 0xE091_0392;

    {
        let mut emulator = Emulator::dummy();

        // 0x0 * 0x0 == 0x0
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_register_value(r3, 0x0);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x0 * 0x0 == 0x0 (carry_flag and overflow_flag set)
        emulator.cpu.set_register_value(r2, 0x0);
        emulator.cpu.set_register_value(r3, 0x0);
        emulator.cpu.set_nzcv(false, false, true, true);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x1 * 0x1 == 0x1
        emulator.cpu.set_register_value(r2, 0x1);
        emulator.cpu.set_register_value(r3, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x0);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x7FFF_FFFF * 0x7FFF_FFFF == (0x1, 0x3FFF_FFFF)
        emulator.cpu.set_register_value(r2, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r3, 0x7FFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x3FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0x8000_0000 * 0x7FFF_FFFF == (0x8000_0000, 0x3FFF_FFFF)
        emulator.cpu.set_register_value(r2, 0x8000_0000);
        emulator.cpu.set_register_value(r3, 0x7FFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x3FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
        assert_eq!(emulator.cpu.get_register_value(r3), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    {
        let mut emulator = Emulator::dummy();

        // 0xFFFF_FFFF * 0xFFFF_FFFF == (0x1, 0xFFFF_FFFE)
        emulator.cpu.set_register_value(r2, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r3, 0xFFFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_register_value(r1), 0xFFFF_FFFE);
        assert_eq!(emulator.cpu.get_register_value(r2), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r3), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }
}
