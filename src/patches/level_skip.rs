use crate::consts::{op, BANK_BASE_ADDR};
use crate::rom::Rom;

/// Pause+Select warp patch.
/// Mirrors the kill-switch structure: only runs when paused,
/// pressing Select triggers a warp ($DFA3).
pub fn apply_pause_select_warp_patch(rom: &mut Rom) {
    let bank = 15;
    let base = rom.bank_base(bank);

    // Hook pause handler at $E039
    let pause_hook = base + ((0xE039u16 - BANK_BASE_ADDR) as usize);
    rom.write_slice(pause_hook, &[op::JSR, 0x40, 0xFE]); // JSR $FE40

    // Routine at $FEE4
    let warp_routine: [u8; 39] = [
        0x20, 0xA8, 0xCB, // JSR $CBA8 (pause toggle housekeeping)
        0xA5, 0x16, // LDA $16 (buttons newly pressed this frame)
        0x29, 0x20, // AND #$20 (Select bit)
        0xF0, 0x0E, // BEQ skip_all (if Select not pressed)
        // --- Check Up ---
        0xA5, 0x19, // LDA $19 (currently held)
        0x29, 0x08, // AND #$08 (Up bit)
        0xF0, 0x09, // BEQ check_down (if Up not pressed)
        0xA9, 0x00, // LDA #$00
        0x8D, 0x20, 0x01, // STA $0120 (clear pause flag)
        0x20, 0xA3, 0xDF, // JSR $DFA3 (warp up)
        0x60, // RTS
        // --- Check Down ---
        0xA5, 0x19, // LDA $19 (reload)
        0x29, 0x04, // AND #$04 (Down bit)
        0xF0, 0x08, // BEQ skip_all (if Down not pressed)
        0xA9, 0x00, // LDA #$00
        0x8D, 0x20, 0x01, // STA $0120 (clear pause flag)
        0x20, 0xB4, 0xDF, // JSR $DFB4 (warp down)
        0x60, // RTS
    ];

    let warp_off = base + ((0xFE40u16 - BANK_BASE_ADDR) as usize);
    rom.write_slice(warp_off, &warp_routine);

    println!("Pause+Select warp patch installed at $FEE4");
}
