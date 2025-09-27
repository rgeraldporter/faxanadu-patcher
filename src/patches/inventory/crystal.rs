use crate::rom::Rom;

/// Patch the Crystal (0x8B) so it's usable with Down+B
pub fn patch_crystal_warp(rom: &mut Rom) {
    let bank = 14;
    let jump_table_cpu = 0xC49D;
    let base = rom.bank_base(bank);

    let table_off = base + (jump_table_cpu - 0x8000) as usize;

    // Crystal is item ID 0x8B
    // Table starts at 0x80, 2 bytes each.
    let index = 0x8B - 0x80;
    let entry_off = table_off + index as usize * 2;

    // Warp-up routine lives at $DFA3 but..
    //  the bank switch process works by jumping to one byte before the instruction
    //  then naturally advancing, so we subtract 1 to reach our target.
    let target: u16 = 0xDFA3 - 1;
    let lo = (target & 0xFF) as u8;
    let hi = (target >> 8) as u8;

    rom.write_byte(entry_off, lo);
    rom.write_byte(entry_off + 1, hi);

    println!(
        "Patched Crystal (0x8B) use-table entry at 0x{:X} -> ${:04X} (warp up)",
        entry_off, target
    );
}

use crate::consts::BANK_BASE_ADDR;

const CRYSTAL_WARP_CPU: u16 = 0xFE67;

pub fn install_crystal_warp_stub(rom: &mut Rom) {
    let stub: [u8; 7] = [
        0xEE, 0x35, 0x04, // INC $0435
        0x20, 0xDC, 0xDA, // JSR $DADC
        0x60, // RTS
    ];

    let off = rom.cpu_to_file_offset(15, CRYSTAL_WARP_CPU, BANK_BASE_ADDR);
    rom.write_slice(off, &stub);

    println!(
        "Installed Crystal warp stub at ${:04X} (bank 15)",
        CRYSTAL_WARP_CPU
    );
}
