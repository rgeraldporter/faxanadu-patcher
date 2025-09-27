pub mod eolis;
pub mod trunk;
use crate::rom::Rom;

use crate::consts::*;
use eolis::*;
use trunk::*;

const START_SCREEN_CPU: u16 = 0xDECB;

/// Write the starting Eolis screen byte at bank 15:$DECB.
pub fn set_start_screen(rom: &mut Rom, screen: EolisScreen) {
    let off = rom.cpu_to_file_offset(TARGET_BANK, START_SCREEN_CPU, BANK_BASE_ADDR);
    rom.write_byte(off, screen as u8);
    println!(
        "Set start screen @${:04X} (bank {}) = 0x{:02X} ({:?})",
        START_SCREEN_CPU, TARGET_BANK, screen as u8, screen
    );
}
