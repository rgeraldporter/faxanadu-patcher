use crate::consts::{BANK_BASE_ADDR, TARGET_BANK};
use crate::rom::Rom;

const START_GOLD_CPU: u16 = 0xDE8D;

// nonfunc - @todo fix this
pub fn set_starting_gold(rom: &mut Rom, amount: u16) {
    let off = rom.cpu_to_file_offset(TARGET_BANK, START_GOLD_CPU, BANK_BASE_ADDR);
    let lo = (amount & 0xFF) as u8;
    let hi = (amount >> 8) as u8;
    rom.write_byte(off, lo);
    rom.write_byte(off + 1, hi);
    println!(
        "Set starting gold @${:04X} (bank {}) = {} (0x{:04X})",
        START_GOLD_CPU, TARGET_BANK, amount, amount
    );
}
