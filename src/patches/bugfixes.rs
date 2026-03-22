use crate::allocator::FreeSpaceAllocator;
use crate::consts::jsr_bytes;
use crate::rom::Rom;

/*
 * Hourglass Fixed
 *
 * In the original game, Hourglass cost half of the player's health. This
 * removes that cost.
 *
 * Original patch by Notlob for Randumizer
 */
pub fn fix_hourglass(rom: &mut Rom) {
    // NOP out where the player's health is halved
    let stub: [u8; 9] = [ 0xEA, 0xEA, 0xEA, 0xEA, 0xEA, 0xEA, 0xEA, 0xEA, 0xEA ]; // NOP ; x9
    let off = rom.cpu_to_file_offset(15, 0xC5D8);
    rom.write_bytes(off, &stub);

    println!("patch: Hourglass no longer costs 50% of the player's health");
}

/*
 * Shield-Ointment Fix
 *
 * In the original game, shields were not ignored when you had ointment.
 * This means getting hit with a spell knocked you back still, making it
 * advantageous for speedrunners to skip equipping shields altogether.
 *
 * This patches the behaviour so that magic passes through shields when
 * ointment is active.
 *
 * Code originally by Notlob for the Randumizer
 */
pub fn shield_ointment_fix(rom: &mut Rom, alloc: &mut FreeSpaceAllocator) {
    // newcode: ignore shield deflection when player has ointment
    let code: [u8; 12] = [
        0xAD, 0x27, 0x04,           // LDA $0427        ; a:DurationOintment
        0x10, 0x04,                 // BPL +4           ; @skip
        0xAD, 0xBF, 0x03,           // LDA $03BF        ; a:SelectedShield
        0x60,                       // RTS
                                    // @skip:
        0xA9, 0x03,                 // LDA #$03
        0x60,                       // RTS
    ];
    let addr = alloc.alloc(14, code.len());
    let off = rom.cpu_to_file_offset(14, addr);
    rom.write_bytes(off, &code);

    // overwrite with JSR to newcode
    let off = rom.cpu_to_file_offset(14, 0x877C);
    rom.write_bytes(off, &jsr_bytes(addr));

    println!("patch: Shields will be ignored when player has ointment invulnerability active");
}

/*
 * Ointment Fix: Sugata
 *
 * In the original game, the Sugata's screenwide "flash" attack actually ignores ointment.
 * For this patch we keep the knockback but skip having any damage if ointment is currently
 * active.
 */
pub fn ointment_sugata_fix(rom: &mut Rom, alloc: &mut FreeSpaceAllocator) {
    let code: [u8; 16] = [
        0xAD, 0x27, 0x04,       // LDA $0427        ; a:DurationOintment
        0x10, 0x0A,             // BPL +10          ; @skipDamage
        0xA9, 0x00,             // LDA #$00
        0x8D, 0xBC, 0x04,       // STA $04BC        ; a:Arg_PlayerHealthDelta_L
        0xA9, 0x0A,             // LDA #$0A
        0x8D, 0xBD, 0x04,       // STA $04BD        ; a:Arg_PlayerHealthDelta_U
                                // @skipDamage:
        0x60,                   // RTS
    ];
    let addr = alloc.alloc(14, code.len());
    let off = rom.cpu_to_file_offset(14, addr);
    rom.write_bytes(off, &code);

    let hook: [u8; 10] = [
        jsr_bytes(addr)[0], jsr_bytes(addr)[1], jsr_bytes(addr)[2],
        0xEA, 0xEA, 0xEA,       // NOP ; x7
        0xEA, 0xEA, 0xEA,
        0xEA,
    ];
    let off = rom.cpu_to_file_offset(14, 0xAB53);
    rom.write_bytes(off, &hook);

    println!("patch: Ointment protects against Sugata's screenwide flash attack")
}

/*
 * Fix Fire Spell animation
 *
 * This fixes a wrong frame definition in the Fire spell animation.
 *
 * Original code by Kaimitai.
 */
pub fn fix_fire_spell_animation(rom: &mut Rom) {
    let off = rom.cpu_to_file_offset(7, 0xA5D3);
    rom.write_bytes(off, &[0x7E]);
    println!("patch: Fixed wrong frame in Fire spell animation");
}

/*
 * Fix Studded Mail Climb Tile
 *
 * This fixes a bug that appears when Studded Mail is equipped without
 * a shield. When the player uses a ladder, one of the tiles is wrong.
 *
 * Original code by Kaimitai.
 */
pub fn fix_studded_mail_climb_tile(rom: &mut Rom) {
    let off = rom.cpu_to_file_offset(7, 0xABF9);
    rom.write_bytes(off, &[0x30]);
    println!("patch: Fixed missing tile in Studded Mail climbing animation (no shield)");
}
