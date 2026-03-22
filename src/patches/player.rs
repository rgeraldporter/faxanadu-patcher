use crate::allocator::FreeSpaceAllocator;
use crate::consts::jsr_bytes;
use crate::rom::Rom;

/*
 * Prevent Knockback on Ladders
 *
 * In Faxanadu, when a player is hit by any source of damage they are knocked
 * back. This includes when on a ladder. This patch changes this behaviour so that
 * the player will "hold on" to the ladder and not be knocked off.
 *
 * Originally written by Notlob for Randumizer.
 */
pub fn no_knockback_on_ladders(rom: &mut Rom, alloc: &mut FreeSpaceAllocator) {
    // newcode: prevent knockback
    let code: [u8; 19] = [
        0x20, 0x52, 0xE7,   // JSR $E752        ; Player_CheckIfOnLadder
        0xA5, 0xA4,         // LDA $A4
        0x29, 0x08,         // AND #$08
        0xF0, 0x05,         // BEQ +5           ; @knockbackPlayer (if not on ladder)
        0xA9, 0x00,         // LDA #$00
        0x85, 0xAA,         // STA $AA
        0x60,
                            // @knockbackPlayer:
        0xA9, 0x08,         // LDA #$08         ; set upper acceleration byte to 8 (knock player back)
        0x85, 0xAA,         // STA $AA
        0x60,               // RTS
    ];
    let addr = alloc.alloc(15, code.len());
    let off = rom.cpu_to_file_offset(15, addr);
    rom.write_bytes(off, &code);

    // overwrite part of `Player_UpdatePosFromKnockback` with JSR to newcode
    let hook: [u8; 4] = [
        jsr_bytes(addr)[0], jsr_bytes(addr)[1], jsr_bytes(addr)[2],
        0xEA,               // NOP
    ];
    let off = rom.cpu_to_file_offset(15, 0xE28C);
    rom.write_bytes(off, &hook);

    println!("patch: Player is no longer knocked off ladder if they take damage");
}

/*
 * Allow Lower Respawn Value
 *
 * In the original game, talking to a Guru in a previous town would not change your
 * spawn point upon death. This makes it so you now have a spawn point equal to the
 * last Guru you spoke to.
 */
pub fn allow_lower_respawn(rom: &mut Rom) {
    // https://chipx86.com/faxanadu/PRG12.html#IScriptAction_SetSpawnPoint
    // removes comparison of current spawn point vs new spawn point
    let stub: [u8; 5] = [
        0xEA, 0xEA, 0xEA, 0xEA, 0xEA,   // NOP      ; x5
    ];

    let off = rom.cpu_to_file_offset(12, 0x8394); // within IScriptAction_SetSpawnPoint
    rom.write_bytes(off, &stub);

    println!("patch: Allows player to set spawn point at guru with lower spawn value than previous (earlier in game)");
}

/*
 * All Killswitch
 *
 * This adds a "killswitch" where you can trigger player death by pressing START, SELECT, then
 * START again. Useful for testing or as a preventitive measure for softlocks.
 *
 * Originally by Notlob for Randumizer
 */
pub fn add_killswitch(rom: &mut Rom, alloc: &mut FreeSpaceAllocator) {
    let code: [u8; 15] = [
        0x20, 0xA8, 0xCB,       // JSR $CBA8        ; Sprites_FlipRanges
        0xA5, 0x19,             // LDA $19          ; Joy1_ChangedButtonMask
        0x29, 0x20,             // AND #$20         ; mask: select button pressed?
        0xF0, 0x05,             // BEQ +5           ; @return
        0xA9, 0x01,             // LDA $01
        0x8D, 0x38, 0x04,       // STA $0438        ; PlayerIsDead
                                // @return:
        0x60,                   // RTS
    ];
    let addr = alloc.alloc(15, code.len());
    let off = rom.cpu_to_file_offset(15, addr);
    rom.write_bytes(off, &code);

    let off = rom.cpu_to_file_offset(15, 0xE039);  // within @_waitForUnpause
    rom.write_bytes(off, &jsr_bytes(addr));

    println!("patch: Player can intentionally die immediately by pausing, pressing select, then unpausing");
}

/*
 * Pendant Quest (private fn)
 *
 * Provides code for using the QUEST_EXTRA flag to determine if the Pendant is "cursed" or not,
 * with variations for 25% or 50% boon/penalty.
 */
fn pendant_quest_inner(rom: &mut Rom, alloc: &mut FreeSpaceAllocator, variant: PendantVariant) {
    let code_50: [u8; 24] = [
        0xAD, 0x2D, 0x04,       // LDA $042D        ; Quests (bitflag)
        0x29, 0x48,             // AND #$48
        0xC9, 0x48,             // CMP #$48
        0xF0, 0x06,             // BEQ +6           ; @attackBuff

                                // @attackCursed:   ; Cursed Pendant: -50% damage
        0xA5, 0x00,             // LDA $00          ; Temp_00 (damage)
        0x4A,                   // LSR A            ; damage/2
        0x85, 0x00,             // STA $00          ; result = 0.5x
        0x60,                   // RTS

                                // @attackBuff:     ; Quest complete: +50% damage
        0xA5, 0x00,             // LDA $00          ; Temp_00 (damage)
        0x4A,                   // LSR A            ; damage/2
        0x18,                   // CLC
        0x65, 0x00,             // ADC $00          ; damage/2 + damage = 1.5x
        0x85, 0x00,             // STA $00
        0x60,                   // RTS
    ];
    let code_25: [u8; 31] = [
        0xAD, 0x2D, 0x04,       // LDA $042D        ; Quests (bitflag)
        0x29, 0x48,             // AND #$48
        0xC9, 0x48,             // CMP #$48
        0xF0, 0x0C,             // BEQ +12          ; @attackBuff

                                // @attackCursed:   ; Cursed Pendant: -25% damage (×0.75)
        0xA5, 0x00,             // LDA $00          ; damage
        0x4A,                   // LSR A            ; damage/2
        0x85, 0x01,             // STA $01          ; save half in Temp_01
        0x4A,                   // LSR A            ; damage/4
        0x18,                   // CLC
        0x65, 0x01,             // ADC $01          ; damage/4 + damage/2 = 0.75x
        0x85, 0x00,             // STA $00
        0x60,                   // RTS

                                // @attackBuff:     ; Quest complete: +25% damage (×1.25)
        0xA5, 0x00,             // LDA $00          ; damage
        0x4A,                   // LSR A            ; damage/2
        0x4A,                   // LSR A            ; damage/4
        0x18,                   // CLC
        0x65, 0x00,             // ADC $00          ; damage/4 + damage = 1.25x
        0x85, 0x00,             // STA $00
        0x60,                   // RTS
    ];
    let (code, label): (&[u8], &str) = match variant {
        PendantVariant::Percent50 => (&code_50, "50"),
        PendantVariant::Percent25 => (&code_25, "25"),
    };
    let addr = alloc.alloc(14, code.len());
    let off = rom.cpu_to_file_offset(14, addr);
    rom.write_bytes(off, code);

    let hook: [u8; 11] = [
        0xF0, 0x09,                 // BEQ +9        ; branch only if we have pendant
        jsr_bytes(addr)[0], jsr_bytes(addr)[1], jsr_bytes(addr)[2],
        0xEA, 0xEA, 0xEA,
        0xEA, 0xEA, 0xEA           // NOP           ; x6
    ];
    let off = rom.cpu_to_file_offset(14, 0x8879);
    rom.write_bytes(off, &hook);

    println!("patch: Pendant is cursed by default, but grants +{}% attack after completing the QUEST_EXTRA flag", label);
}

pub enum PendantVariant {
    Percent25,
    Percent50,
}

pub fn pendant_quest_25(rom: &mut Rom, alloc: &mut FreeSpaceAllocator) {
    pendant_quest_inner(rom, alloc, PendantVariant::Percent25);
}

pub fn pendant_quest_50(rom: &mut Rom, alloc: &mut FreeSpaceAllocator) {
    pendant_quest_inner(rom, alloc, PendantVariant::Percent50);
}

/*
 * Allow Menu on First Screen
 *
 * In the original game, the Select button (player menu) is disabled on the
 * very first screen of Eolis (Area 0, Screen 0). This patch removes that
 * restriction so the menu can be opened anywhere.
 *
 * Originally by Notlob for Randumizer
 */
pub fn allow_menu_on_first_screen(rom: &mut Rom) {
    // NOP out the BEQ at $E01C that skips the menu when
    // Area_CurrentArea == 0 && Area_CurrentScreen == 0
    let off = rom.cpu_to_file_offset(15, 0xE01C);
    rom.write_bytes(off, &[0xEA, 0xEA]);

    println!("patch: Select button menu is available on the first screen of Eolis");
}
