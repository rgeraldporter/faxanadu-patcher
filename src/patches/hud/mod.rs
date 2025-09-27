use crate::consts::{BANK_BASE_ADDR, TARGET_BANK};
use crate::rom::Rom;
use crate::subroutine::Subroutine;

pub mod frame_counter;
pub mod health;
pub mod health_frac;
pub mod level_display;
pub mod mana;
pub mod ointment;
pub mod player_stats_hook;
pub mod screen_id;

/// Substitute the original JSR call sites so they jump into our HUD routines.
pub fn substitute_all_jsr(rom: &mut Rom) {
    fn substitute_jsr(rom: &mut Rom, destination: u16, target: u16) {
        let off = rom.cpu_to_file_offset(TARGET_BANK, destination, BANK_BASE_ADDR);
        let buf = [0x20, (target & 0xFF) as u8, (target >> 8) as u8]; // JSR target
        rom.write_slice(off, &buf);
        println!(
            "Substitute JSR at CPU ${:04X} (file offset 0x{:X}) -> target ${:04X}",
            destination, off, target
        );
    }

    substitute_jsr(rom, 0xC8BF, 0xFCD0); // ointment decrement
    substitute_jsr(rom, 0xE036, 0xFE20); // frame counter (pause loop)
    substitute_jsr(rom, 0xDB4D, 0xFE20); // frame counter (main game loop)
    substitute_jsr(rom, 0xC1EF, 0xFD30); // screen id
    substitute_jsr(rom, 0xFA7E, 0xFD90); // health
}

/// Apply all HUD patches directly at fixed addresses
pub fn apply_all_hud_patches(rom: &mut Rom) {
    let routines: [(&str, u16, Subroutine); 7] = [
        ("oint_dec", 0xFCD0, ointment::build_ointment_decrement()),
        ("oint_draw", 0xFD00, ointment::build_ointment_draw()),
        ("screen", 0xFD30, screen_id::build_screen_id()),
        ("level_sub", 0xFD60, level_display::build_level_display()),
        ("health", 0xFD90, health::build_health()),
        ("health_f", 0xFEA0, health_frac::build_health_frac()),
        //("mana", 0xFE60, mana::build_mana()),
        ("frame", 0xFE20, frame_counter::build_frame_counter()),
    ];

    for (name, target_cpu, sub) in routines {
        let off = rom.cpu_to_file_offset(TARGET_BANK, target_cpu, BANK_BASE_ADDR);
        let bytes = sub.bytes();
        rom.write_slice(off, bytes);
        println!(
            "HUD: wrote {} at ${:04X} ({} bytes)",
            name,
            target_cpu,
            bytes.len()
        );
    }

    // Player stats hook (JSR $FE60 at $FA8B)
    player_stats_hook::build_player_stats_hook();
}
