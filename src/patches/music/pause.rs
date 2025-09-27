use crate::consts::{BANK_BASE_ADDR, TARGET_BANK};
use crate::rom::Rom;

pub fn apply_pause_music_patch(rom: &mut Rom) {
    // Patch 1: 0x14020 D0 -> F0
    let off1 = 0x0014_020usize;
    let before1 = rom.read_byte(off1);
    if before1 != 0xD0 {
        println!(
            "Warning: expected D0 at 0x{:06X}, found {:02X}",
            off1, before1
        );
    }
    rom.write_byte(off1, 0xF0);

    // Patch 2: 0x14045 20 4A 80 -> EA EA EA
    let off2 = 0x0014_045usize;
    let before2 = [
        rom.read_byte(off2),
        rom.read_byte(off2 + 1),
        rom.read_byte(off2 + 2),
    ];
    if before2 != [0x20, 0x4A, 0x80] {
        println!(
            "Warning: expected 20 4A 80 at 0x{:06X}, found {:02X} {:02X} {:02X}",
            off2, before2[0], before2[1], before2[2]
        );
    }
    rom.write_slice(off2, &[0xEA, 0xEA, 0xEA]);

    println!("Applied pause-music IPS-equivalent patch at 0x14020 and 0x14045.");
}
